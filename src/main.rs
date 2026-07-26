use crate::{models::User, network::{run_client, run_server}};

pub mod models;
pub mod network;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    
    let user = User::new(0, &args[1].as_str());
    let my_port = &args[2];
    let peer_port = &args[3];

    let addr = format!("127.0.0.1:{my_port}");
    let peer_addr = format!("127.0.0.1:{peer_port}");

    tokio::join!(run_server(addr.as_str()), run_client(&user,&peer_addr.as_str()));

    Ok(())
}
