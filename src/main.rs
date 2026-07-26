pub mod models;
pub mod network;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("server") => network::run_server("127.0.0.1:8080").await?,
        Some("client") => network::run_client("127.0.0.1:8080").await?,
        _ => println!("Usage: cargo run -- [server|client]"),
    }

    Ok(())
}
