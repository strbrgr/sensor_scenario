use std::{
    error::Error,
    io::Read,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use iggy::prelude::{
    Client, CompressionAlgorithm, DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME, IggyClient,
    IggyExpiry, MaxTopicSize, StreamClient, TopicClient, UserClient,
};
use tracing::{info, warn};

const STREAM_NAME: &str = "sample-stream";
const TOPIC_NAME: &str = "sample-topic";

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
        println!("Received message:: {}", msg);
    }
}

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
