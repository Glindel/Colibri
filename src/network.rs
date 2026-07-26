use std::time::Duration;

use crate::models::{Message, User};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

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
            Ok(message) => {
                println!("Message received:: {message}")
            }
            Err(e) => {
                println!("Error received : {e}")
            }
        };
    }
}

pub async fn run_client(user: &User, addr: &str) -> anyhow::Result<()> {
    loop {
        match TcpStream::connect(addr).await {
            Ok(mut stream) => {
                println!("Connecté au serveur {addr}");
                if let Err(e) = start_message_delivery(user, &mut stream).await {
                    println!("Connexion perdue: {e}, tentative de reconnexion");
                }
            }
            Err(_) => {
                print!("Connexion échoué! Nouvelle tentative dans 5s...");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

pub async fn start_message_delivery(user: &User, stream: &mut TcpStream) -> anyhow::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let message = Message::new(
            user.id(),
            format!("Ceci est petit ping de {}", user.username()).as_str(),
        );
        let mut body = serde_json::to_string(&message)?;
        body.push('\n');
        let data = body.as_bytes();

        stream.write_all(data).await?;
        println!("Datas send");
    }
}
