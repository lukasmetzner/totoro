use anyhow::Result;
use totoro::server::TotoroServer;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let server = TotoroServer::new(Default::default()).await?;
    server.run().await?;
    Ok(())
}
