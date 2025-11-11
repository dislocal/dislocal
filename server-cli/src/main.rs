use iroh::Endpoint;
use server_cli::{Result, instrument};

#[tokio::main]
async fn main() -> Result<()> {
    instrument::init();

    let endpoint = Endpoint::bind().await?;

    println!("Hello, Async World");
    Ok(())
}
