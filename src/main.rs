use std::time::Duration;

use crate::models::{Channel, LoginState, User, Message};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};

pub mod models;

#[tokio::main]
async fn main() {
    let user1 = User::new(1, String::from("Alice"));
    show_status(&user1, LoginState::LoggedIn);
    show_status(
        &user1,
        LoginState::Waiting(String::from("Vérification du pair...")),
    );

    let (sender, mut receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel(100);


    tokio::spawn(async move {
       loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let message = Message::new(user1.id(), "Ceci est un petit ping");
            if let Err(_) = sender.send(message).await {
                println!("Receiver dropped");
            }
        }
    });

    while let Some(message) = receiver.recv().await {
        println!("{message}");
    }


}

fn show_status(user: &User, state: LoginState) {
    println!("{} (#{}) - {}", user.username(), user.id(), state.message())
}
