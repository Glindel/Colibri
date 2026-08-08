use crate::{models::User, network::{run_client, run_server}};

pub mod models;
pub mod network;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    
    let user = User::new(0, &args[1]);
    let my_port: &String = &args[2];
    let peer_ports = &args[3..];

    let addr = format!("127.0.0.1:{my_port}");

    let peer_addrs: Vec<String> = peer_ports
    .iter()
    .map(|port| format!("127.0.0.1:{port}"))
    .collect(); 

    for peer_addr in peer_addrs {
        let user_clone = user.clone();
        tokio::spawn(async move {
            run_client(&user_clone, &peer_addr).await
        });
    }

    run_server(addr.as_str()).await?;
    Ok(())
}
