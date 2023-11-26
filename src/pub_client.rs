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
    let mut test_counter: usize = 0;

    // Registration
    let mut reg_buffer: Vec<u8> = vec![0; MAX_BUFFER_SIZE];
    reg_buffer[1] = 1;
    stream.write_all(&reg_buffer).await?;
    stream.read_exact(&mut reg_buffer).await?;
    match reg_buffer[0] {
        1 => info!("Registered successfully"),
        _ => panic!("Could not register"),
    }

    loop {
        let message = format!("test counter value: {}", test_counter);
        let mut buffer: Vec<u8> = vec![0; MAX_BUFFER_SIZE];
        buffer[0] = 2;
        buffer[1..message.len() + 1].copy_from_slice(message.as_bytes());
        stream.write_all(&buffer).await?;
        info!("Sent message to queue: {}", message);
        test_counter += 1;
    }
}
