use crate::models::{User, LoginState};

pub mod models;

fn main() {
    let user1 = User::new(1, String::from("Alice"));
    show_status(&user1, LoginState::LoggedIn);
    show_status(&user1, LoginState::Waiting(String::from("Vérification du pair...")));

}

fn show_status(user: &User, state: LoginState) {
    println!( "{} (#{}) - {}", user.username(), user.id(), state.message())
} 