use std::{env, io::ErrorKind, process::Command};

/// Get the public key from the file system.
pub fn get_public_key(username: &str) -> Result<String, ErrorKind> {
    let root = env::current_dir().unwrap();
    let public_key_path = root.join(format!("keys\\{}.pem", &username));
    let result = std::fs::read_to_string(&public_key_path);
    match result {
        Ok(public_key_content) => Ok(public_key_content),

        Err(_) => {
            let output = Command::new("cmd")
                .arg("runas")
                .arg("/user:Administrator")
                .arg("/c")
                .arg("type")
                .arg(&public_key_path)
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
                    } else {
                        println!("Error on give administrator permission: {}", output.status);
                        return Err(ErrorKind::NotFound);
                    }
                }
                Err(e) => {
                    println!("Error on give administrator permission: {}", e);
                    return Err(ErrorKind::NotFound);
                }
            }
        }
    }
}
