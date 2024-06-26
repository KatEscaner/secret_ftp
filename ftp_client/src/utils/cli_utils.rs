use clap::Parser;
use clap_derive::Parser;

use super::fs_utils::{check_if_file_exists, get_file};

/// Represents the command-line arguments.
#[derive(Parser)]
#[command(name = "FTP Client")]
#[command(about = "A simple FTP client to connect, authenticate, and perform file operations", long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long)]
    pub username: String,

    #[arg(short, long)]
    pub private_key_path: String,
}

/// Enum representing the various commands supported by the CLI.
#[derive(Parser)]
pub enum Commands {
    List,
    UploadFile { path: String },
    Download { filename: String },
    Delete { filename: String },
    Quit,
    Help,
}

impl Commands {
    /// Parses a string into a `Commands` enum.
    pub fn from_str(input: &str) -> Option<Self> {
        let mut parts = input.trim().splitn(2, ' ');
        let command = parts.next()?;
        let argument = parts.next();

        match command.to_lowercase().as_str() {
            "list" => Some(Commands::List),
            "upload" => {
                if let Some(argument) = argument {
                    let mut split = argument.splitn(2, ' ');
                    let path = split.next()?.to_string();
                    if !check_if_file_exists(path.clone()) {
                        println!("File does not exist.");
                        return None;
                    }
                    Some(Commands::UploadFile { path: path })
                } else {
                    println!("No path provided.");
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
