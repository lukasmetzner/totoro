use anyhow::Result;
use totoro::client::TotoroSubscriber;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut subscriber = TotoroSubscriber::new(Default::default()).await?; 
    loop {
        let message = subscriber.consume().await?;
        info!("Received message: {}", message);
    }
}
