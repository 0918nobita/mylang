use language_server::{send_notification, wait_for_initialize_request};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = tokio::spawn(async { wait_for_initialize_request().await });
    send_notification().await;
    handle.await?
}
