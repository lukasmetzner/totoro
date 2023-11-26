use anyhow::Result;
use std::{sync::Arc, vec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};
use tracing::{error, info};
use tracing_subscriber;

type SharedReceiver = Arc<Mutex<Receiver<String>>>;

const MAX_CHANNEL_SIZE: usize = 64000;
const MAX_BUFFER_SIZE: usize = 1024;
const LISTEN_ADDRESS: &str = "127.0.0.1:8000";

enum ClientType {
    SUB = 0,
    PUB = 1,
}

struct QueueServer {
    listener: TcpListener,
    shared_sender: Sender<String>,
    shared_receiver: SharedReceiver,
}

impl QueueServer {
    pub async fn new() -> Result<QueueServer> {
        let listener = TcpListener::bind(LISTEN_ADDRESS).await?;
        let (shared_sender, rx) = mpsc::channel::<String>(MAX_CHANNEL_SIZE);
        let shared_receiver: SharedReceiver = Arc::new(Mutex::new(rx));
        info!(
            "Create new queue server with listen address: {}",
            LISTEN_ADDRESS
        );
        Ok(QueueServer {
            listener,
            shared_receiver,
            shared_sender,
        })
    }

    async fn register_client(&self, stream: &mut TcpStream) -> Result<ClientType> {
        let mut buffer: Vec<u8> = vec![0; MAX_BUFFER_SIZE];
        stream.read_exact(&mut buffer).await?;
        if buffer[0] != 0 {
            error!("Unkown Opcode.");
        }

        let client_type = match buffer[1] {
            0 => ClientType::SUB,
            1 => ClientType::PUB,
            _ => panic!("Unkown client type."),
        };

        buffer = vec![0; MAX_BUFFER_SIZE];
        buffer[0] = 1;
        stream.write_all(&buffer).await?;

        Ok(client_type)
    }

    async fn handle_subscriber(stream: &mut TcpStream, receiver: SharedReceiver) -> Result<()> {
        info!("Registered new subscriber");
        loop {
            let mut lrx = receiver.lock().await;
            let message = lrx.recv().await;
            match message {
                Some(message) => {
                    let mut buffer: Vec<u8> = vec![0; MAX_BUFFER_SIZE];
                    buffer[0] = 2;
                    buffer[1..message.len() + 1].copy_from_slice(message.as_bytes());
                    stream.write_all(&buffer).await?;
                    info!("Sent following message to subscriber: {}", message);
                }
                None => continue,
            }
        }
    }

    async fn handle_publisher(stream: &mut TcpStream, sender: Sender<String>) -> Result<()> {
        info!("Registered new publisher");
        loop {
            let mut buffer: Vec<u8> = vec![0; MAX_BUFFER_SIZE];
            stream.read_exact(&mut buffer).await?;
            match buffer[0] {
                2 => {
                    let message = String::from_utf8(buffer[1..].to_vec())?;
                    info!("Pushed message to queue: {}", message);
                    sender.send(message).await?;
                }
                _ => panic!("Unkown opcode"),
            }
        }
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;
            let client_type = self.register_client(&mut stream).await?;
            match client_type {
                ClientType::SUB => {
                    let shared_receiver = self.shared_receiver.clone();
                    tokio::spawn(async move {
                        QueueServer::handle_subscriber(&mut stream, shared_receiver)
                            .await
                            .unwrap();
                    });
                }
                ClientType::PUB => {
                    let shared_sender = self.shared_sender.clone();
                    tokio::spawn(async move {
                        QueueServer::handle_publisher(&mut stream, shared_sender)
                            .await
                            .unwrap();
                    });
                }
            };
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let server = QueueServer::new().await?;
    server.run().await?;
    Ok(())
}
