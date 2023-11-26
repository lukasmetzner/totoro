const DEFAULT_MAX_CHANNEL_SIZE: usize = 64000;
const DEFAULT_MAX_BUFFER_SIZE: usize = 1024;
const DEFAULT_LISTEN_ADDRESS: &str = "127.0.0.1:8000";

#[derive(Clone)]
pub struct TotoroConfig {
    pub max_channel_size: usize,
    pub max_buffer_size: usize,
    pub listen_address: String,
}

impl Default for TotoroConfig {
    fn default() -> Self {
        Self {
            max_channel_size: DEFAULT_MAX_CHANNEL_SIZE,
            max_buffer_size: DEFAULT_MAX_BUFFER_SIZE,
            listen_address: DEFAULT_LISTEN_ADDRESS.to_string(),
        }
    }
}