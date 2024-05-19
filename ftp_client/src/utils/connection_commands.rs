use std::error::Error;
use tokio::io::AsyncBufReadExt;
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;

async fn get_response(stream: &mut TcpStream) -> String {
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response).await.unwrap();
    response
}

async fn send_command(stream: &mut TcpStream, command: &str) {
    stream.write_all(command.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

pub async fn connect() -> Result<TcpStream, Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:2121").await?;
    println!("{}", get_response(&mut stream).await);
    return Ok(stream);
}

pub async fn login(
    stream: &mut TcpStream,
    username: &str,
    password: &str,
) -> Result<String, Box<dyn Error>> {
    let mut stream = stream;
    send_command(&mut stream, &format!("USER {}\r\n", username)).await;
    println!("{}", get_response(&mut stream).await);

    send_command(stream, &format!("PASS {}\r\n", password)).await;
    let login_response = get_response(stream).await;
    println!("{}", login_response);
    return Ok(login_response);
}
