use std::{
    error::Error,
    io::{self, Read, Write},
};
use tokio::io::AsyncBufReadExt;
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;

const USERNAME: &str = "public_key";
const PASSWORD: &str = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDqxP0ehzFgHmE6vsZcEHWUFWFO2M3CmLUkQbBr/7BxkTiwJs3S2L/pSGJ3GUE9P0TSV+Tz0iYScRLV5d+tqsQVb1fC5ZYm0Ej3JpvbS8VbQccN+TaumauCukwN3mASahJOYP9/Qnl4CYuCXuVS+yCGNM3dxds65+PD/3DMjYBZFcMw5Q5DX32cWobPbwQu4jwDk1AtdqTMmBdXNhy2r5IqKfrjdwol9ezcicuFuNxpe6G8TlTG10GSLIj/+zf+qx1OR4cvzfRXuHXBh2SzAZYgrzzRGq16nRc+g2PMQBXr7nFjQyfGz0G7uCMzvnub+0dhjvabE85zh1LZwUiNUNQp escaner@DESKTOP-5EENPHJ";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:2121").await?;
    println!("{}", get_response(&mut stream).await);

    send_command(&mut stream, &format!("USER {}\r\n", USERNAME)).await;
    println!("{}", get_response(&mut stream).await);

    send_command(&mut stream, &format!("PASS {}\r\n", PASSWORD)).await;

    let login_response = get_response(&mut stream).await;
    println!("{}", login_response);

    if login_response.starts_with("230") {
        println!("Login successful!");
    } else {
        println!("Login failed.");
    }

    Ok(())
}

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
