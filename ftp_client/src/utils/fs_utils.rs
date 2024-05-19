use std::{env, process::Command};

pub fn get_public_key() -> String {
    let root = env::current_dir().unwrap();
    let public_key_path = root.join(format!("{}.pem", "public_key"));
    let result = std::fs::read_to_string(&public_key_path);
    match result {
        Ok(public_key_content) => public_key_content,

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

pub fn get_private_key() -> String {
    let root = env::current_dir().unwrap();
    let private_key_path = root.join(format!("{}.pem", "private_key"));
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
