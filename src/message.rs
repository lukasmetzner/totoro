use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::config::TotoroConfig;

#[derive(Clone)]
pub enum ClientType {
    Subscriber,
    Publisher,
}

pub enum Message {
    Registration(ClientType),
    RegistrationAck,
    Data(String),
}

impl Message {
    pub async fn from_stream(stream: &mut TcpStream, totoro_config: &TotoroConfig) -> Result<Self> {
        let mut buffer: Vec<u8> = vec![0; totoro_config.max_buffer_size];
        stream.read_exact(&mut buffer).await?;

        let message = match buffer[0] {
            0 => Message::Registration(match buffer[1] {
                0 => ClientType::Subscriber,
                1 => ClientType::Publisher,
                _ => panic!("Unkown client type"),
            }),
            1 => Message::RegistrationAck,
            2 => Message::Data(String::from_utf8(buffer[1..].to_vec())?),
            _ => panic!("Unkown opcode"),
        };

        Ok(message)
    }

    pub async fn send_message(
        &self,
        stream: &mut TcpStream,
        totoro_config: &TotoroConfig,
    ) -> Result<()> {
        let buffer = self.to_buffer(totoro_config);
        stream.write_all(&buffer).await?;
        Ok(())
    }

    pub fn to_buffer(&self, totoro_config: &TotoroConfig) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; totoro_config.max_buffer_size];
        match self {
            Message::Registration(client_type) => {
                buffer[1] = match client_type {
                    ClientType::Subscriber => 0,
                    ClientType::Publisher => 1,
                }
            }
            Message::RegistrationAck => buffer[0] = 1,
            Message::Data(message) => {
                buffer[0] = 2;
                if message.len() > totoro_config.max_buffer_size - 1 {
                    panic!("Message exceeded max buffer size");
                }
                buffer[1..message.len() + 1].copy_from_slice(message.as_bytes());
            }
        };
        buffer
    }
}
