use clap::Parser;
use clap_derive::Parser;
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

mod utils;
use crate::utils::{connection_commands, fs_utils, openssl_utils};

#[derive(Parser)]
#[command(name = "FTP Client")]
#[command(about = "A simple FTP client to connect, authenticate, and perform file operations", long_about = None)]
struct Cli {
    #[arg(short, long)]
    username: String,

    #[arg(short, long)]
    private_key_path: String,
}

#[derive(Parser)]
enum Commands {
    List,
    Upload { filename: String, content: String },
    Download { filename: String },
    Delete { filename: String },
    Quit,
    Help,
}

impl Commands {
    fn from_str(input: &str) -> Option<Self> {
        let mut parts = input.trim().splitn(2, ' ');
        let command = parts.next()?;
        let argument = parts.next();

        match command.to_lowercase().as_str() {
            "list" => Some(Commands::List),
            "upload" => {
                if let Some(argument) = argument {
                    let mut split = argument.splitn(2, ' ');
                    let filename = split.next()?.to_string();
                    let content = split.next()?.to_string();
                    Some(Commands::Upload { filename, content })
                } else {
                    println!("No filename or content provided.");
                    None
                }
            }
            "download" => {
                if let Some(filename) = argument {
                    Some(Commands::Download {
                        filename: filename.to_string(),
                    })
                } else {
                    println!("No filename provided");
                    None
                }
            }
            "delete" => {
                if let Some(filename) = argument {
                    Some(Commands::Delete {
                        filename: filename.to_string(),
                    })
                } else {
                    println!("No filename provided");
                    None
                }
            }
            "quit" => Some(Commands::Quit),
            "help" => Some(Commands::Help),
            _ => {
                println!("Invalid command.");
                None
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let username = &args.username;
    let private_key_path = &args.private_key_path;

    let private_key = fs_utils::get_private_key(private_key_path.to_owned());

    let mut stream = connection_commands::connect().await?;

    let text = username.to_string();
    let text_signed = openssl_utils::sign_message(&private_key, &text)?;

    let login_response = connection_commands::login(&mut stream, username, &text_signed).await?;

    if login_response.starts_with("230") {
        println!("Login successful!");
    } else {
        println!("Login failed.");
        return Ok(());
    }

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut input = String::new();

    loop {
        input.clear();
        println!("Enter command (list, upload, download, delete, quit, help): ");
        reader.read_line(&mut input).await?;
        let input = input.trim();

        if let Some(command) = Commands::from_str(input) {
            match command {
                Commands::List => {
                    let files = connection_commands::list_files(&mut stream).await?;
                    println!("Files:\n{}", files);
                }
                Commands::Upload { filename, content } => {
                    let response = connection_commands::upload_file(
                        &mut stream,
                        &filename,
                        content.as_bytes(),
                    )
                    .await?;
                    println!("Upload response: {}", response);
                }
                Commands::Download { filename } => {
                    let content =
                        connection_commands::download_file(&mut stream, &filename).await?;
                    println!("Downloaded content:\n{}", String::from_utf8_lossy(&content));
                }
                Commands::Delete { filename } => {
                    let response = connection_commands::delete_file(&mut stream, &filename).await?;
                    println!("Delete response: {}", response);
                }
                Commands::Quit => {
                    let response = connection_commands::quit(&mut stream).await?;
                    println!("Quit response: {}", response);
                    break;
                }
                Commands::Help => {
                    println!("Available commands:");
                    println!("  list - List files on the server");
                    println!(
                        "  upload [filename] [content] - Upload a file with specified content"
                    );
                    println!("  download [filename] - Download a file from the server");
                    println!("  delete [filename] - Delete a file from the server");
                    println!("  quit - Quit the FTP client");
                    println!("  help - Show this help message");
                }
            }
        }
    }

    Ok(())
}
