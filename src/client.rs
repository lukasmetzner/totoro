use anyhow::Result;
use tokio::net::TcpStream;

use crate::{config::TotoroConfig, message::Message};

pub struct TotoroPublisher {
    totoro_config: TotoroConfig,
    stream: TcpStream,
}

impl TotoroPublisher {
    pub async fn new(totoro_config: TotoroConfig) -> Result<Self> {
        let mut stream = TcpStream::connect(&totoro_config.listen_address).await?;
        Message::Registration(crate::message::ClientType::Publisher)
            .send_message(&mut stream, &totoro_config)
            .await?;
        let ack = Message::from_stream(&mut stream, &totoro_config).await?;
        match ack {
            Message::RegistrationAck => Ok(Self {
                totoro_config,
                stream
            }),
            _ => panic!("Did not receive ack message"),
        }
    }

    pub async fn publish(&mut self, data: String) -> Result<()> {
        Message::Data(data)
            .send_message(&mut self.stream, &self.totoro_config)
            .await
    }
}

pub struct TotoroSubscriber {
    totoro_config: TotoroConfig,
    stream: TcpStream,
}

impl TotoroSubscriber {
    pub async fn new(totoro_config: TotoroConfig) -> Result<Self> {
        let mut stream = TcpStream::connect(&totoro_config.listen_address).await?;
        Message::Registration(crate::message::ClientType::Subscriber)
            .send_message(&mut stream, &totoro_config)
            .await?;
        let ack = Message::from_stream(&mut stream, &totoro_config).await?;
        match ack {
            Message::RegistrationAck => Ok(Self {
                totoro_config,
                stream
            }),
            _ => panic!("Did not receive ack message"),
        }
    }

    pub async fn consume(&mut self) -> Result<String> {
        let message = Message::from_stream(&mut self.stream, &self.totoro_config).await?;
        let data = match message {
            Message::Data(data) => data,
            _ => panic!("Did not receive Data message"),
        };
        Ok(data)
    }
}
