use clap::Parser;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{get, post, routes, State};
use std::error::Error;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use utils::connection_commands::FileEntry;

mod utils;
use crate::utils::cli_utils::Cli;
use crate::utils::fs_utils::get_file;
use crate::utils::{
    cli_utils::{self, Commands},
    connection_commands, fs_utils, openssl_utils,
};

#[derive(Deserialize)]
struct UploadData {
    filename: String,
    content: String,
}

struct UserContext {
    username: String,
    text_signed: String,
}

#[get("/list")]
async fn list_files_handler(user_context: &State<Arc<Mutex<UserContext>>>) -> Json<Vec<FileEntry>> {
    let user_context = user_context.lock().await;
    let username = &user_context.username;
    let text_signed = &user_context.text_signed;

    let mut stream = connection_commands::connect()
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    connection_commands::login(&mut stream, &username, &text_signed).await;

    let files = connection_commands::list_files(&mut stream).await;
    match files {
        Ok(files) => Json(files),
        Err(e) => {
            eprintln!("Error listing files: {}", e);
            Json(Vec::new())
        }
    }
}

#[post("/upload", format = "json", data = "<data>")]
async fn upload_file_handler(
    data: Json<UploadData>,
    user_context: &State<Arc<Mutex<UserContext>>>,
) -> Json<String> {
    let user_context = user_context.lock().await;
    let username = &user_context.username;
    let text_signed = &user_context.text_signed;

    let mut stream = connection_commands::connect()
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    connection_commands::login(&mut stream, &username, &text_signed).await;

    let response =
        connection_commands::upload_file(&mut stream, &data.filename, data.content.as_bytes())
            .await;
    match response {
        Ok(response) => Json(response),
        Err(e) => Json(e.to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let username = args.username.clone();
    let private_key_path = args.private_key_path;

    let private_key = fs_utils::get_private_key(private_key_path);
    let text_signed = openssl_utils::sign_message(&private_key, &username)?;
    let text_signed_copy = text_signed.clone();

    let user_context = Arc::new(Mutex::new(UserContext {
        username,
        text_signed,
    }));

    let rocket_handle = {
        let user_context = user_context.clone();
        tokio::spawn(async move {
            rocket::build()
                .manage(user_context)
                .mount("/", routes![list_files_handler, upload_file_handler])
                .launch()
                .await
                .unwrap();
        })
    };

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut input = String::new();

    loop {
        let mut stream = connection_commands::connect()
            .await
            .map_err(|e| e.to_string())?;

        let _ = connection_commands::login(
            &mut stream,
            args.username.clone().as_str(),
            text_signed_copy.as_str(),
        )
        .await;

        input.clear();
        println!("Enter command (list, upload, download, delete, quit, help): ");
        reader.read_line(&mut input).await?;
        let input = input.trim();

        if let Some(command) = Commands::from_str(input) {
            match command {
                Commands::List => {
                    let files = connection_commands::list_files(&mut stream).await?;
                    for file in files {
                        println!("{:?}", file);
                    }
                }
                Commands::UploadFile { path } => {
                    let content: String = get_file(path.clone());

                    let response =
                        connection_commands::upload_file(&mut stream, &path, content.as_bytes())
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
                    println!("list - List files");
                    println!("upload <path> - Upload a file");
                    println!("download <filename> - Download a file");
                    println!("delete <filename> - Delete a file");
                    println!("quit - Quit the program");
                }
            }
        }
    }

    rocket_handle.await.unwrap();

    Ok(())
}
