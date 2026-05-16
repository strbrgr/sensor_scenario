use iggy::prelude::*;
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

const STREAM_NAME: &str = "sample-stream";
const TOPIC_NAME: &str = "sample-topic";
const PARTITION_ID: u32 = 0;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let root_username =
        env::var("IGGY_ROOT_USERNAME").unwrap_or_else(|_| DEFAULT_ROOT_USERNAME.to_string());
    let root_password =
        env::var("IGGY_ROOT_PASSWORD").map_err(|_| "IGGY_ROOT_PASSWORD must be set (see .env)")?;

    let client = IggyClient::default();
    client.connect().await?;
    client.login_user(&root_username, &root_password).await?;
    consume_messages(&client).await
}

async fn consume_messages(client: &IggyClient) -> Result<(), Box<dyn Error>> {
    let interval = Duration::from_millis(500);
    info!(
        "Messages will be consumed from stream: {}, topic: {}, partition: {} with interval {} ms.",
        STREAM_NAME,
        TOPIC_NAME,
        PARTITION_ID,
        interval.as_millis()
    );

    let mut offset = 0;
    let messages_per_batch = 10;
    let consumer = Consumer::default();
    loop {
        let polled_messages = client
            .poll_messages(
                &STREAM_NAME.try_into()?,
                &TOPIC_NAME.try_into()?,
                Some(PARTITION_ID),
                &consumer,
                &PollingStrategy::offset(offset),
                messages_per_batch,
                false,
            )
            .await?;

        if polled_messages.messages.is_empty() {
            info!("No messages found.");
            sleep(interval).await;
            continue;
        }

        offset += polled_messages.messages.len() as u64;
        for message in polled_messages.messages {
            handle_message(&message)?;
        }
        sleep(interval).await;
    }
}

fn handle_message(message: &IggyMessage) -> Result<(), Box<dyn Error>> {
    // The payload can be of any type as it is a raw byte array. In this case it's a simple string.
    let payload = std::str::from_utf8(&message.payload)?;
    info!(
        "Handling message at offset: {}, payload: {}...",
        message.header.offset, payload
    );
    Ok(())
}
