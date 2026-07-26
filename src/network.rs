use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::models::{Message, User};

pub async fn run_server(addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Serveur en écoute sur {}", addr);

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        println!("Nouveau pair connecté {addr}");
        tokio::spawn(async move {
            handle_connexion(socket).await;
        });
    }
}

async fn handle_connexion(socket: TcpStream) {
    let reader = BufReader::new(socket);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        match serde_json::from_str::<Message>(line.as_str()) {
            Ok(message) => { println!("Message received:: {message}") },
            Err(e) => { println!("Error received : {e}")},
        };
    }
}

pub async fn run_client(addr: &str) -> anyhow::Result<()> {

    let mut stream = TcpStream::connect(addr).await?;
    println!("Connecté au serveur {addr}");
    let user = User::new(1, "Meretoast");

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let message = Message::new(user.id(), "Ceci est petit ping de Meretoast");
        let mut body = serde_json::to_string(&message)?;
        body.push('\n');
        let data = body.as_bytes();
        
        match stream.write_all(data).await {
            Ok(()) => println!("Datas send"),
            Err(e) => println!("Error happened: {e}"),
        };
    }
}