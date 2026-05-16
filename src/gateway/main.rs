use std::{env, error::Error, str::FromStr, sync::Arc};

use iggy::prelude::{
    Client, CompressionAlgorithm, DEFAULT_ROOT_USERNAME, Identifier, IggyClient, IggyDuration,
    IggyExpiry, IggyMessage, MaxTopicSize, MessageClient, Partitioning, StreamClient, TopicClient,
    UserClient,
};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const STREAM_NAME: &str = "sample-stream";
const TOPIC_NAME: &str = "sample-topic";
const PARTITION_ID: u32 = 0;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let root_username = env::var("IGGY_ROOT_USERNAME")
        .unwrap_or_else(|_| DEFAULT_ROOT_USERNAME.to_string());
    let root_password = env::var("IGGY_ROOT_PASSWORD")
        .map_err(|_| "IGGY_ROOT_PASSWORD must be set (see .env)")?;

    let client = Arc::new(IggyClient::default());
    client.connect().await?;
    client.login_user(&root_username, &root_password).await?;

    let (stream_id, topic_id) = init_system(&client).await?;
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let client = Arc::clone(&client);

        tokio::spawn(async move {
            let _ = handle_client(stream, client, stream_id, topic_id).await;
        });
    }
}

async fn init_system(client: &IggyClient) -> Result<(u32, u32), Box<dyn Error>> {
    let stream_ident = Identifier::named(STREAM_NAME)?;
    let topic_ident = Identifier::named(TOPIC_NAME)?;

    let stream = match client.create_stream(STREAM_NAME).await {
        Ok(stream) => {
            info!("Stream was created.");
            stream
        }
        Err(_) => {
            warn!("Stream already exists and will not be created again.");
            client
                .get_stream(&stream_ident)
                .await?
                .ok_or("stream not found after create-already-exists")?
        }
    };

    let topic = match client
        .create_topic(
            &stream_ident,
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
                .get_topic(&stream_ident, &topic_ident)
                .await?
                .ok_or("topic not found after create-already-exists")?
        }
    };

    Ok((stream.id, topic.id))
}

async fn handle_client(
    mut stream: TcpStream,
    client: Arc<IggyClient>,
    stream_id: u32,
    topic_id: u32,
) -> Result<(), Box<dyn Error>> {
    let duration = IggyDuration::from_str("500ms")?;
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
    let partitioning = Partitioning::partition_id(PARTITION_ID);
    let mut messages = Vec::new();

    loop {
        let mut incoming_message_len_buf = [0u8; 4];

        if stream
            .read_exact(&mut incoming_message_len_buf)
            .await
            .is_err()
        {
            return Ok(());
        }

        let incoming_message_len = u32::from_be_bytes(incoming_message_len_buf) as usize;

        let mut buf = vec![0u8; incoming_message_len];
        stream.read_exact(&mut buf).await?;

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

            info!("Sent {messages_per_batch} message(s).");
            messages.clear();
        }
    }
}
