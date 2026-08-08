use std::time::SystemTime;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct User {
    id: u64,
    username: String,
}

impl User {
    pub fn new(id: u64, username: &str) -> User {
        User { id, username: String::from(username) }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

pub enum LoginState {
    LoggedIn,
    LoggedOut,
    Waiting(String),
}

impl LoginState {
    pub fn message(&self) -> String {
         match self {
            LoginState::LoggedIn => String::from("Connecté"),
            LoginState::LoggedOut => String::from("Déconnecté"),
            LoginState::Waiting(reason) => {
                format!("En attente : {}", reason)
            }
        }
    }
}

pub struct Channel {
    id: u64, 
    name: String,
    messages: Vec<Message>
}

impl Channel {
    pub fn new(id: u64, name: &str) -> Channel {
        Channel { id, name: String::from(name), messages: Vec::new() } 
    }

    pub fn post_message(&mut self, author_id: u64, content: &str) {
        let message = Message::new(author_id, content);
        self.messages.push(message);
    }

    pub fn history(&self) -> &[Message] {
        &self.messages
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    author_id: u64,
    content: String,
    timestamp: u64,
}

impl Message {
    pub fn new(author_id: u64, content: &str) -> Message {
        let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Clock may run backward")
        .as_secs();

        Message { author_id, content: String::from(content), timestamp }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (timestamp: {})", self.content, self.timestamp)
    }
}


