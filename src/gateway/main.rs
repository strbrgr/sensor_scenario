use std::{
    error::Error,
    io::Read,
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use iggy::prelude::{
    Client, CompressionAlgorithm, DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME, IggyClient,
    IggyExpiry, IggyMessage, MaxTopicSize, MessageClient, Partitioning, StreamClient, TopicClient,
    UserClient,
};
use tokio::time::sleep;
use tracing::{info, warn};

const STREAM_NAME: &str = "sample-stream";
const TOPIC_NAME: &str = "sample-topic";
const PARTITION_ID: u32 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let client = Arc::new(IggyClient::default());
    client.connect().await?;
    client
        .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
        .await?;
    init_system(&client).await;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let client = Arc::clone(&client);

        tokio::spawn(async move {
            let _ = handle_client(stream, client);
        });
    }

    Ok(())
}

async fn init_system(client: &IggyClient) {
    match client.create_stream(STREAM_NAME).await {
        Ok(_) => info!("Stream was created."),
        Err(_) => warn!("Stream already exists and will not be created again."),
    }

    match client
        .create_topic(
            &STREAM_NAME.try_into().unwrap(),
            TOPIC_NAME,
            1,
            CompressionAlgorithm::default(),
            None,
            IggyExpiry::NeverExpire,
            MaxTopicSize::ServerDefault,
        )
        .await
    {
        Ok(_) => info!("Topic was created."),
        Err(_) => warn!("Topic already exists and will not be created again."),
    }
}

fn handle_client(mut stream: TcpStream, client: Arc<IggyClient>) -> std::io::Result<()> {
    loop {
        let mut len_buf = [0u8; 4];

        if stream.read_exact(&mut len_buf).is_err() {
            return Ok(());
        }

        let msg_len = u32::from_be_bytes(len_buf) as usize;

        let mut buf = vec![0u8; msg_len];
        stream.read_exact(&mut buf)?;

        let msg: String = serde_json::from_slice(&buf)?;
        let msg = IggyMessage::from(msg);
        let mut msgs = vec![msg];
        let partitioning = Partitioning::partition_id(PARTITION_ID);

        client.send_messages(
            &STREAM_NAME.try_into().unwrap(),
            &TOPIC_NAME.try_into().unwrap(),
            &partitioning,
            &mut msgs,
        );

        // println!("Received message:: {}", msg);
    }
}

async fn produce_messages(client: &IggyClient) -> Result<(), Box<dyn Error>> {
    let interval = Duration::from_millis(500);
    info!(
        "Messages will be sent to stream: {}, topic: {}, partition: {} with interval {} ms.",
        STREAM_NAME,
        TOPIC_NAME,
        PARTITION_ID,
        interval.as_millis()
    );

    let mut current_id = 0;
    let messages_per_batch = 10;
    let partitioning = Partitioning::partition_id(PARTITION_ID);
    loop {
        let mut messages = Vec::new();
        for _ in 0..messages_per_batch {
            current_id += 1;
            let payload = format!("message-{current_id}");
            let message = IggyMessage::from_str(&payload)?;
            messages.push(message);
        }
        client
            .send_messages(
                &STREAM_NAME.try_into().unwrap(),
                &TOPIC_NAME.try_into().unwrap(),
                &partitioning,
                &mut messages,
            )
            .await?;
        info!("Sent {messages_per_batch} message(s).");
        sleep(interval).await;
    }
}
