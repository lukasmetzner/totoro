<h1>Under Construction 🚧</h1>

<p align="center">
  <img src="assets/totoro-logo-no-bg.png" alt="Logo" width="200"/>
</p>
<h1 align="center">Totoro</h1>
<h3 align="center">A single topic in-memory message queue server written in Rust</h3>

## Description 
- One instance has exactly one queue
    - No multi topic functionality
- Max messages in queue: 64000

## Run Server
```
cargo run --bin totoro
```

## Run Examples
```
cargo run --bin sub-client
cargo run --bin pub-client
```

## Example Publisher
``` Rust
use anyhow::Result;
use totoro::client::TotoroPublisher;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut publisher = TotoroPublisher::new(Default::default()).await?; 
    let mut msg_counter = 0;
    loop {
        let message = format!("Message counter: {}", msg_counter);
        publisher.publish(message.clone()).await?;
        msg_counter += 1;
        info!("Sent message: {}", message);
    }
}
```

## Example Subscriber
``` Rust
use anyhow::Result;
use totoro::client::TotoroSubscriber;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut subscriber = TotoroSubscriber::new(Default::default()).await?; 
    loop {
        let message = subscriber.consume().await?;
        info!("Received message: {}", message);
    }
}
```

## About the name
ChatGPT came up with the name (and logo above) **TotoroTransfer**, which I shortened to **Totoro**. Here is the explanation from ChatGPT:

The name "TotoroTransfer" for your Rust message queue project is inspired by Totoro, the iconic character from the Studio Ghibli film "My Neighbor Totoro." Here's a breakdown of the thought process behind this name:

**Totoro's Popularity and Appeal**: Totoro is a beloved character known for his friendly and comforting presence. By naming your message queue after Totoro, you're tapping into this positive association, suggesting that your project is approachable, user-friendly, and reliable.

**Symbolism of Comfort and Reliability**: Totoro in the film is a figure of comfort, support, and reliability for the main characters. Similarly, "TotoroTransfer" implies a system that users can rely on for efficient and dependable message handling, much like how Totoro is a dependable character in the film.

**Ease and Simplicity**: The film "My Neighbor Totoro" is celebrated for its simplicity and the warm, easy-going nature of its narrative. This can reflect on your project as well, suggesting that "TotoroTransfer" offers a straightforward, hassle-free experience in message queuing and processing.

**Cultural Recognition**: Totoro is a globally recognized character, and using this name can make your project more memorable and engaging, especially for fans of anime and Studio Ghibli. It's a name that's likely to stand out and be remembered.

**Friendly and Welcoming Tone**: Finally, the name conveys a sense of warmth and friendliness. In the often complex and technical world of software, a name like "TotoroTransfer" can make your project seem more accessible and less intimidating to potential users.

In summary, "TotoroTransfer" leverages the character's attributes of friendliness, reliability, and simplicity, suggesting a message queue that is both efficient and user-friendly, while also being memorable and culturally engaging.

