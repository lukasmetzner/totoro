use anyhow::Result;

use totoro::builder::TotoroBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let server = TotoroBuilder::new().build().await?;
    server.run().await?;
    Ok(())
}
