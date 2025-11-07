#[cfg(feature = "server")]
pub mod server;

pub mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::main().await
}
