use std::{env, fs, process::Command};

/// Reads the private key from a file.
pub fn get_private_key(private_key_path: String) -> String {
    let result = std::fs::read_to_string(&private_key_path);
    match result {
        Ok(public_key_content) => public_key_content,

        Err(_) => {
            let output = Command::new("cmd")
                .arg("runas")
                .arg("/user:Administrator")
                .arg("/c")
                .arg("type")
                .arg(&private_key_path)
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        return String::from_utf8_lossy(&output.stdout)
                            .to_string()
                            .trim()
                            .to_string();
                    } else {
                        println!("Error on give administrator permission: {}", output.status);
                        return String::new();
                    }
                }
                Err(e) => {
                    println!("Error on give administrator permission: {}", e);
                    return String::new();
                }
            }
        }
    }
}

/// Reads the content of a file.
pub fn get_file(file_path: String) -> String {
    let result = std::fs::read_to_string(&file_path);
    match result {
        Ok(file_content) => file_content,

        Err(_) => {
            let output = Command::new("cmd")
                .arg("runas")
                .arg("/user:Administrator")
                .arg("/c")
                .arg("type")
                .arg(&file_path)
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        return String::from_utf8_lossy(&output.stdout).to_string();
                    } else {
                        println!("Error on give administrator permission: {}", output.status);
                        return String::new();
                    }
                }
                Err(e) => {
                    println!("Error on give administrator permission: {}", e);
                    return String::new();
                }
            }
        }
    }
}

/// Checks if a file exists.
pub fn check_if_file_exists(file_path: String) -> bool {
    std::path::Path::new(&file_path).exists()
}

/// Writes content to a file in the downloads directory.
pub fn write_file_in_downloads(file_path: String, content: &[u8]) -> std::io::Result<()> {
    let root = env::current_dir();
    match root {
        Ok(root) => {
            let root = root.to_str().unwrap();
            let downloads_path = format!("{}\\downloads", root);

            if !fs::metadata(&downloads_path).is_ok() {
                fs::create_dir(&downloads_path)?;
            }

            let file_path = format!("{}\\{}", downloads_path, file_path);
            fs::write(file_path, content)?;
        }
        Err(e) => {
            println!("Error getting current directory: {}", e);
        }
    }
    Ok(())
}
