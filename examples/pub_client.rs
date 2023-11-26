use anyhow::Result;
use totoro::client::TotoroPublisher;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut publisher = TotoroPublisher::new(Default::default()).await?; 
    let mut msg_counter = 0;
    loop {
        let message = format!("Message counter: {}", msg_counter);
        publisher.publish(message.clone()).await?;
        msg_counter += 1;
        info!("Sent message: {}", message);
    }
}
