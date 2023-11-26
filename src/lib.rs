use anyhow::Result;
use config::TotoroConfig;
use message::{ClientType, Message};
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};
use tracing::info;

type SharedReceiver = Arc<Mutex<Receiver<String>>>;

pub mod builder;
pub mod config;
pub mod message;

pub struct Totoro {
    totoro_config: TotoroConfig,
    listener: TcpListener,
    shared_sender: Sender<String>,
    shared_receiver: SharedReceiver,
}

impl Totoro {
    pub async fn new(totoro_config: TotoroConfig) -> Result<Totoro> {
        let listener = TcpListener::bind(&totoro_config.listen_address).await?;
        let (shared_sender, rx) = mpsc::channel::<String>(totoro_config.max_channel_size);
        let shared_receiver: SharedReceiver = Arc::new(Mutex::new(rx));
        info!(
            "Created new totoro server with listen address: {}",
            totoro_config.listen_address
        );
        Ok(Totoro {
            totoro_config,
            listener,
            shared_receiver,
            shared_sender,
        })
    }

    async fn register_client(&self, stream: &mut TcpStream) -> Result<ClientType> {
        let message = Message::from_stream(stream, &self.totoro_config).await?;
        let client_type = match message {
            Message::Registration(client_type) => {
                Message::RegistrationAck
                    .send_message(stream, &self.totoro_config)
                    .await?;
                client_type
            }
            _ => panic!("Expected message type Registration here"),
        };

        Ok(client_type)
    }

    async fn handle_subscriber(
        stream: &mut TcpStream,
        receiver: SharedReceiver,
        totoro_config: TotoroConfig,
    ) -> Result<()> {
        info!("Registered new subscriber");
        loop {
            let mut lrx = receiver.lock().await;
            let message = lrx.recv().await;
            match message {
                Some(message) => {
                    Message::Data(message)
                        .send_message(stream, &totoro_config)
                        .await?;
                    info!("Sent message to subscriber");
                }
                None => continue,
            }
        }
    }

    async fn handle_publisher(
        stream: &mut TcpStream,
        sender: Sender<String>,
        totoro_config: TotoroConfig,
    ) -> Result<()> {
        info!("Registered new publisher");
        loop {
            let message = Message::from_stream(stream, &totoro_config).await?;
            match message {
                Message::Data(data) => sender.send(data).await?,
                _ => panic!("Expected message type Data here"),
            }
            info!("Pushed message to queue");
        }
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;
            let client_type = self.register_client(&mut stream).await?;
            let totoro_config = self.totoro_config.clone();
            match client_type {
                ClientType::Subscriber => {
                    let shared_receiver = self.shared_receiver.clone();
                    tokio::spawn(async move {
                        Totoro::handle_subscriber(&mut stream, shared_receiver, totoro_config)
                            .await
                            .unwrap();
                    });
                }
                ClientType::Publisher => {
                    let shared_sender = self.shared_sender.clone();
                    tokio::spawn(async move {
                        Totoro::handle_publisher(&mut stream, shared_sender, totoro_config)
                            .await
                            .unwrap();
                    });
                }
            };
        }
    }
}
