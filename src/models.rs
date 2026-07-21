pub struct User {
    id: u64,
    username: String,
}

impl User {
    pub fn new(id: u64, username: String) -> User {
        User { id, username }
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
