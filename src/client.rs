use anyhow::Result;
use tokio::net::TcpStream;

use crate::{
    config::TotoroConfig,
    message::{ClientType, Message},
};

async fn register_helper(
    config: &TotoroConfig,
    client_type: ClientType,
) -> Result<(TcpStream, Message)> {
    let mut stream = TcpStream::connect(&config.listen_address).await?;

    // Register at server as publisher or subscriber
    Message::Registration(client_type)
        .send_message(&mut stream, &config)
        .await?;
    let message = Message::from_stream(&mut stream, &config).await?;

    Ok((stream, message))
}

pub struct TotoroPublisher {
    totoro_config: TotoroConfig,
    stream: TcpStream,
}

impl TotoroPublisher {
    pub async fn new(totoro_config: TotoroConfig) -> Result<Self> {
        let (stream, message) = register_helper(&totoro_config, ClientType::Publisher).await?;
        match message {
            Message::RegistrationAck => Ok(Self {
                totoro_config,
                stream,
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
        let (stream, message) = register_helper(&totoro_config, ClientType::Subscriber).await?;
        match message {
            Message::RegistrationAck => Ok(Self {
                totoro_config,
                stream,
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
