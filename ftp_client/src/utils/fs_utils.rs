use std::{env, process::Command};

pub fn get_public_key(public_key_path: String) -> String {
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

pub fn check_if_file_exists(file_path: String) -> bool {
    std::path::Path::new(&file_path).exists()
}
