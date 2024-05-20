use clap::Parser;
use clap_derive::Parser;

#[derive(Parser)]
#[command(name = "FTP Client")]
#[command(about = "A simple FTP client to connect, authenticate, and perform file operations", long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long)]
    pub username: String,

    #[arg(short, long)]
    pub private_key_path: String,
}

#[derive(Parser)]
pub enum Commands {
    List,
    Upload { filename: String, content: String },
    Download { filename: String },
    Delete { filename: String },
    Quit,
    Help,
}

impl Commands {
    pub fn from_str(input: &str) -> Option<Self> {
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
