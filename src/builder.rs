use anyhow::Result;
use crate::config::TotoroConfig;
use crate::server::TotoroServer;

pub struct TotoroBuilder {
    totoro_config: TotoroConfig
}

impl TotoroBuilder {
    pub fn new() -> TotoroBuilder {
        Self { totoro_config: Default::default() }
    }

    pub fn max_channel_size(mut self, max_channel_size: usize) -> Self {
        self.totoro_config.max_channel_size = max_channel_size;
        self
    }

    pub fn max_buffer_size(mut self, max_buffer_size: usize) -> Self {
        self.totoro_config.max_buffer_size = max_buffer_size;
        self
    }

    pub fn listen_address(mut self, listen_address: String) -> Self {
        self.totoro_config.listen_address = listen_address;
        self
    }

    pub async fn build(self) -> Result<TotoroServer> {
        TotoroServer::new(self.totoro_config).await
    }
}