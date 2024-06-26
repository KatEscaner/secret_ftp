use std::error::Error;
use tokio::io::AsyncBufReadExt;
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;

/// Represents a file entry in the FTP server's directory listing.
#[derive(serde::Serialize, Debug, Clone)]
pub struct FileEntry {
    permissions: String,
    links: u32,
    owner: String,
    group: String,
    size: u64,
    month: String,
    day: u32,
    time: String,
    name: String,
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

/// Connects to the FTP server.
pub async fn connect() -> Result<TcpStream, Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:2121").await?;
    get_response(&mut stream).await;
    Ok(stream)
}

/// Logs into the FTP server.
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

/// Lists files on the FTP server.
pub async fn list_files(stream: &mut TcpStream) -> Result<Vec<FileEntry>, Box<dyn Error>> {
    send_command(stream, "PASV\r\n").await;
    let pasv_response = get_response(stream).await;

    let (ip, port) = parse_pasv_response(&pasv_response)?;

    let mut data_stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;

    send_command(stream, "LIST\r\n").await;

    let mut files = String::new();
    data_stream.read_to_string(&mut files).await?;

    let file_entries = parse_file_entries(&files)?;
    Ok(file_entries)
}

fn parse_file_entries(list_output: &str) -> Result<Vec<FileEntry>, Box<dyn Error>> {
    let mut entries = Vec::new();

    for line in list_output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 9 {
            let entry = FileEntry {
                permissions: parts[0].to_string(),
                links: parts[1].parse().unwrap_or(0),
                owner: parts[2].to_string(),
                group: parts[3].to_string(),
                size: parts[4].parse().unwrap_or(0),
                month: parts[5].to_string(),
                day: parts[6].parse().unwrap_or(0),
                time: parts[7].to_string(),
                name: parts[8..].join(" "),
            };
            entries.push(entry);
        }
    }

    Ok(entries)
}

/// Uploads a file to the FTP server.
pub async fn upload_file(
    stream: &mut TcpStream,
    path: &str,
    content: &[u8],
) -> Result<String, Box<dyn Error>> {
    send_command(stream, "PASV\r\n").await;
    let pasv_response = get_response(stream).await;

    let (ip, port) = parse_pasv_response(&pasv_response)?;

    let mut data_stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;

    let filename = path.split('/').last().unwrap();

    send_command(stream, &format!("STOR {}\r\n", filename)).await;

    let response = get_response(stream).await;
    if !response.starts_with("150") {
        return Err(Box::from("Failed to start upload"));
    }

    data_stream.write_all(content).await?;
    data_stream.flush().await?;

    Ok("Upload successful".to_string())
}

/// Downloads a file from the FTP server.
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

/// Deletes a file from the FTP server.
pub async fn delete_file(stream: &mut TcpStream, filename: &str) -> Result<String, Box<dyn Error>> {
    send_command(stream, &format!("DELE {}\r\n", filename)).await;
    let response = get_response(stream).await;
    Ok(response)
}

/// Sends the QUIT command to the FTP server.
pub async fn quit(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    send_command(stream, "QUIT\r\n").await;
    let response = get_response(stream).await;
    Ok(response)
}

fn parse_pasv_response(response: &str) -> Result<(String, u16), Box<dyn Error>> {
    let start = response.find('(').expect("Invalid PASV response") + 1;
    let end = response.rfind(')').expect("Invalid PASV response");
    let fields: Vec<&str> = response[start..end].split(',').collect();

    let ip = format!("{}.{}.{}.{}", fields[0], fields[1], fields[2], fields[3]);
    let port = (fields[4].parse::<u16>()? << 8) + fields[5].parse::<u16>()?;

    Ok((ip, port))
}
