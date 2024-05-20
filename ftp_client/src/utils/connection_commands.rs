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
    get_response(&mut stream).await;
    Ok(stream)
}

pub async fn login(
    stream: &mut TcpStream,
    username: &str,
    password: &str,
) -> Result<String, Box<dyn Error>> {
    send_command(stream, &format!("USER {}\r\n", username)).await;
    get_response(stream).await;

    send_command(stream, &format!("PASS {}\r\n", password)).await;
    let login_response = get_response(stream).await;
    Ok(login_response)
}

pub async fn list_files(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    send_command(stream, "PASV\r\n").await;
    let pasv_response = get_response(stream).await;

    let (ip, port) = parse_pasv_response(&pasv_response)?;

    let mut data_stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;

    send_command(stream, "LIST\r\n").await;

    let mut files = String::new();
    data_stream.read_to_string(&mut files).await?;

    Ok(files)
}

pub async fn upload_file(
    stream: &mut TcpStream,
    filename: &str,
    content: &[u8],
) -> Result<String, Box<dyn Error>> {
    send_command(stream, "PASV\r\n").await;
    let pasv_response = get_response(stream).await;

    let (ip, port) = parse_pasv_response(&pasv_response)?;

    let mut data_stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;

    send_command(stream, &format!("STOR {}\r\n", filename)).await;

    let response = get_response(stream).await;
    if !response.starts_with("150") {
        return Err(Box::from("Failed to start upload"));
    }

    data_stream.write_all(content).await?;
    data_stream.flush().await?;

    Ok("Upload successful".to_string())
}

pub async fn download_file(
    stream: &mut TcpStream,
    filename: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    send_command(stream, "PASV\r\n").await;
    let pasv_response = get_response(stream).await;

    let (ip, port) = parse_pasv_response(&pasv_response)?;

    let mut data_stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;

    send_command(stream, &format!("RETR {}\r\n", filename)).await;

    let mut content = Vec::new();
    data_stream.read_to_end(&mut content).await?;

    Ok(content)
}

pub async fn delete_file(stream: &mut TcpStream, filename: &str) -> Result<String, Box<dyn Error>> {
    send_command(stream, &format!("DELE {}\r\n", filename)).await;
    let response = get_response(stream).await;
    Ok(response)
}

pub async fn quit(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    send_command(stream, "QUIT\r\n").await;
    let response = get_response(stream).await;
    Ok(response)
}

fn parse_pasv_response(response: &str) -> Result<(String, u16), Box<dyn Error>> {
    let start = response.find('(').ok_or("Invalid PASV response")? + 1;
    let end = response.rfind(')').ok_or("Invalid PASV response")?;
    let fields: Vec<&str> = response[start..end].split(',').collect();

    let ip = format!("{}.{}.{}.{}", fields[0], fields[1], fields[2], fields[3]);
    let port = (fields[4].parse::<u16>()? << 8) + fields[5].parse::<u16>()?;

    Ok((ip, port))
}
