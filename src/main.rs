use crate::models::{Channel, LoginState, User};

pub mod models;
pub mod test;

fn main() {
    let user1 = User::new(1, String::from("Alice"));
    show_status(&user1, LoginState::LoggedIn);
    show_status(
        &user1,
        LoginState::Waiting(String::from("Vérification du pair...")),
    );

    let mut channel = Channel::new(0, "general");
    channel.post_message(user1.id(), "Petit test 1");
    channel.post_message(user1.id(), "Petit test2");
    channel.post_message(user1.id(), "Petit test3");

    for message in channel.history() {
        println!("{message}");
    }
}

fn show_status(user: &User, state: LoginState) {
    println!("{} (#{}) - {}", user.username(), user.id(), state.message())
}
