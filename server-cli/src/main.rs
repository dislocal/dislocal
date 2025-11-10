use server_cli::{Result, instrument};

#[tokio::main]
async fn main() -> Result<()> {
    instrument::init();

    println!("Hello, Async World");
    Ok(())
}
