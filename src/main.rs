use anyhow;
use aws_config::SdkConfig;
use aws_sdk_sqs::Client;
use aws_sdk_sqs::operation::create_queue::CreateQueueOutput;
use aws_sdk_sqs::operation::send_message::SendMessageOutput;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load env");
    let sdk_config = aws_config::load_from_env().await;
    let sqs_client = aws_sdk_sqs::Client::new(&sdk_config);
    let created_queue = create_queue(sqs_client.clone(), "queue-test-from-rs-1235").await;

    match created_queue {
        Ok(ref c) => println!("Successfully created queue: {:?}", c),
        Err(ref e) => println!("Failed to created queue: {:?}", e),
    }

    let queue_url = created_queue.unwrap().queue_url.unwrap();

    let send_result = send_message_queue(sqs_client.clone(), queue_url).await;

    match send_result {
        Ok(s) => println!("Successfully sent message: {:?}", s),
        Err(e) => println!("Failed to send message: {:?}", e),
    }
}

async fn create_queue(sqs_client: Client, queue_name: &str) -> anyhow::Result<CreateQueueOutput> {
    let create_result= sqs_client.create_queue()
        .queue_name(queue_name)
        .send()
        .await?;

    Ok(create_result)
}

async fn send_message_queue(sqs_client: Client, url: String) -> anyhow::Result<SendMessageOutput> {
    let result = sqs_client.send_message()
        .queue_url(url)
        .message_body("TEST MESSAGE")
        .send()
        .await?;

    Ok(result)
}