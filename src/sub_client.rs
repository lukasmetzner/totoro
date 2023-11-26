use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;

const MAX_BUFFER_SIZE: usize = 1024;
const CONNECT_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut stream = TcpStream::connect(CONNECT_ADDRESS).await?;

    // Registration
    let mut reg_buffer: Vec<u8> = vec![0; MAX_BUFFER_SIZE];
    stream.write_all(&reg_buffer).await?;
    stream.read_exact(&mut reg_buffer).await?;

    loop {
        let mut buffer: Vec<u8> = vec![0; MAX_BUFFER_SIZE];
        stream.read_exact(&mut buffer).await?;
        let message = String::from_utf8(buffer)?;
        info!("Received message: {}", message);
    }
}
