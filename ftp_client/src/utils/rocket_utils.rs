use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{delete, fs, get, post, routes, State};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::UserContext;

use super::connection_commands::{self, FileEntry};
use super::fs_utils::{self, get_file};

#[derive(Deserialize)]
pub struct UploadData {
    filename: String,
    content: String,
}

#[derive(Deserialize)]
pub struct UploadFileData {
    path: String,
}

#[get("/list")]
pub async fn list_files_handler(
    user_context: &State<Arc<Mutex<UserContext>>>,
) -> Json<Vec<FileEntry>> {
    let user_context = user_context.lock().await;
    let username = &user_context.username;
    let text_signed = &user_context.text_signed;

    let mut stream = connection_commands::connect()
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    connection_commands::login(&mut stream, &username, &text_signed)
        .await
        .unwrap();

    let files = connection_commands::list_files(&mut stream).await;
    match files {
        Ok(files) => Json(files),
        Err(e) => {
            eprintln!("Error listing files: {}", e);
            Json(Vec::new())
        }
    }
}

#[post("/upload-file", format = "json", data = "<data>")]
pub async fn upload_file_handler(
    data: Json<UploadFileData>,
    user_context: &State<Arc<Mutex<UserContext>>>,
) -> Json<String> {
    let user_context = user_context.lock().await;
    let username = &user_context.username;
    let text_signed = &user_context.text_signed;

    let mut stream = connection_commands::connect()
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    connection_commands::login(&mut stream, &username, &text_signed)
        .await
        .unwrap();

    let content: String = get_file(data.path.clone());

    let response =
        connection_commands::upload_file(&mut stream, &data.path, content.as_bytes()).await;
    match response {
        Ok(response) => Json(response),
        Err(e) => Json(e.to_string()),
    }
}

#[get("/download/<filename>")]
pub async fn download_file_handler(
    filename: String,
    user_context: &State<Arc<Mutex<UserContext>>>,
) -> Result<Json<String>, String> {
    let user_context = user_context.lock().await;
    let username = &user_context.username;
    let text_signed = &user_context.text_signed;

    let mut stream = connection_commands::connect()
        .await
        .map_err(|e| e.to_string())?;

    connection_commands::login(&mut stream, username, text_signed)
        .await
        .map_err(|e| e.to_string())?;

    let content = connection_commands::download_file(&mut stream, &filename).await;
    match content {
        Ok(content) => {
            fs_utils::write_file(filename.clone(), &content).unwrap();
            Ok(Json(String::from_utf8_lossy(&content).to_string()))
        }
        Err(e) => Err(e.to_string()),
    }
}

#[delete("/delete/<filename>")]
pub async fn delete_file_handler(
    filename: String,
    user_context: &State<Arc<Mutex<UserContext>>>,
) -> Result<Json<String>, String> {
    let user_context = user_context.lock().await;
    let username = &user_context.username;
    let text_signed = &user_context.text_signed;

    let mut stream = connection_commands::connect()
        .await
        .map_err(|e| e.to_string())?;

    connection_commands::login(&mut stream, username, text_signed)
        .await
        .map_err(|e| e.to_string())?;

    let response = connection_commands::delete_file(&mut stream, &filename).await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(e.to_string()),
    }
}
