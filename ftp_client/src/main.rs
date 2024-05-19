use std::error::Error;

mod utils;

use crate::utils::{connection_commands, fs_utils, openssl_utils};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = "paco";
    let private_key = fs_utils::get_private_key();

    let mut stream = connection_commands::connect().await?;

    let text = &username.to_string();
    let text_signed = openssl_utils::sign_message(&private_key, text)?;

    let login_response = connection_commands::login(&mut stream, username, &text_signed).await?;

    if login_response.starts_with("230") {
        println!("Login successful!");
    } else {
        println!("Login failed.");
    }
    Ok(())
}
