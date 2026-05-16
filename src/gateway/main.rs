use std::{
    error::Error,
    io::Read,
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use iggy::prelude::{
    Client, CompressionAlgorithm, DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME, Identifier,
    IggyClient, IggyDuration, IggyExpiry, IggyMessage, MaxTopicSize, MessageClient, Partitioning,
    StreamClient, TopicClient, UserClient,
};
use tokio::time::sleep;
use tracing::{info, warn};

const STREAM_NAME: &str = "sample-stream";
const TOPIC_NAME: &str = "sample-topic";
const PARTITION_ID: u32 = 1;
const BATCHES_LIMIT: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    // Incoming Tcp messages
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Producer client
    let client = Arc::new(IggyClient::default());
    client.connect().await?;
    client
        .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
        .await?;
    let (stream_id, topic_id) = init_system(&client).await;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let client = Arc::clone(&client);

        tokio::spawn(async move {
            let _ = handle_client(stream, client, stream_id, topic_id).await;
        });
    }

    Ok(())
}

async fn init_system(client: &IggyClient) -> (u32, u32) {
    let stream = match client.create_stream(STREAM_NAME).await {
        Ok(stream) => {
            info!("Stream was created.");
            stream
        }
        Err(_) => {
            warn!("Stream already exists and will not be created again.");
            client
                .get_stream(&Identifier::named(STREAM_NAME).unwrap())
                .await
                .unwrap()
                .expect("Failed to get stream")
        }
    };

    let topic = match client
        .create_topic(
            &Identifier::named(STREAM_NAME).unwrap(),
            TOPIC_NAME,
            1,
            CompressionAlgorithm::default(),
            None,
            IggyExpiry::NeverExpire,
            MaxTopicSize::ServerDefault,
        )
        .await
    {
        Ok(topic) => {
            info!("Topic was created.");
            topic
        }
        Err(_) => {
            warn!("Topic already exists and will not be created again.");
            client
                .get_topic(
                    &Identifier::named(STREAM_NAME).unwrap(),
                    &Identifier::named(TOPIC_NAME).unwrap(),
                )
                .await
                .unwrap()
                .expect("Failed to get topic")
        }
    };

    (stream.id, topic.id)
}

async fn handle_client(
    mut stream: TcpStream,
    client: Arc<IggyClient>,
    stream_id: u32,
    topic_id: u32,
) -> Result<(), Box<dyn Error>> {
    let duration = IggyDuration::from_str("500ms")?;
    let mut interval = tokio::time::interval(duration.get_duration());
    info!(
        "Messages will be sent to stream: {} ({}), topic: {} ({}), partition: {} with interval {}.",
        STREAM_NAME,
        stream_id,
        TOPIC_NAME,
        topic_id,
        PARTITION_ID,
        duration.as_human_time_string()
    );

    let messages_per_batch = 10;
    let mut sent_batches = 0;
    let partitioning = Partitioning::partition_id(PARTITION_ID);
    let mut messages = Vec::new();

    loop {
        if sent_batches == BATCHES_LIMIT {
            info!("Sent {sent_batches} batches of messages, exiting.");
            return Ok(());
        }
        interval.tick().await;

        let mut incoming_message_len_buf = [0u8; 4];

        if stream.read_exact(&mut incoming_message_len_buf).is_err() {
            return Ok(());
        }

        let incoming_message_len = u32::from_be_bytes(incoming_message_len_buf) as usize;

        let mut buf = vec![0u8; incoming_message_len];
        stream.read_exact(&mut buf)?;

        let message: String = serde_json::from_slice(&buf)?;
        let message = IggyMessage::from(message);
        messages.push(message);

        if messages.len() == messages_per_batch {
            client
                .send_messages(
                    &Identifier::named(STREAM_NAME).unwrap(),
                    &Identifier::named(TOPIC_NAME).unwrap(),
                    &partitioning,
                    &mut messages,
                )
                .await?;

            sent_batches += 1;
            info!("Sent {messages_per_batch} message(s).");

            messages.clear();
        }
    }
}
