use dialoguer::{theme::ColorfulTheme, Input, Password};

use config::Config;

mod auth;
mod config;

fn main() {
    let mut conf = Config::load("config.ini");

    if let Some((username, password)) = conf.get_user_pass() {
        authenticate(username, password);
    } else {
        ask_for_user_pass(&mut conf)
    }
}

fn ask_for_user_pass(conf: &mut Config) {
    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your username")
        .interact_text()
        .unwrap();

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();

    conf.set_user_pass(&username, &password);
    conf.save();
    authenticate(username, password);
}

fn authenticate(username: String, password: String) {
    println!("Authenticating...");
    let response = auth::login(&username, &password);
    println!("{:#?}", response);
}
