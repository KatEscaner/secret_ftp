use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials, DefaultUser};
use libunftp::Server;
use std::env;
use std::fs;
use std::process::{Command, Stdio};
use std::sync::Arc;
use unftp_sbe_fs::ServerExt;

#[derive(Debug)]
struct PublicKeyAuthenticator;

#[async_trait]
impl Authenticator<DefaultUser> for PublicKeyAuthenticator {
    async fn authenticate(
        &self,
        username: &str,
        password: &Credentials,
    ) -> Result<DefaultUser, AuthenticationError> {
        println!("Authenticating user: {}", username);

        let ftp_home = env::current_dir().unwrap();

        let public_key_path = ftp_home.join(format!("{}.pub", username));
        println!("Public key path: {:?}", public_key_path);

        let result = std::fs::read_to_string(&public_key_path);
        match result {
            Ok(public_key_content) => Ok(DefaultUser),

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
                            // Ahora podemos leer la salida del proceso de ejecución
                            if let Ok(output_str) = String::from_utf8(output.stdout) {
                                let password_trimmed = password
                                    .password
                                    .to_owned()
                                    .unwrap()
                                    .chars()
                                    .filter(|c| !c.is_control())
                                    .collect::<String>();
                                let output_trimmed = output_str
                                    .chars()
                                    .filter(|c| !c.is_control())
                                    .collect::<String>();

                                if password_trimmed == output_trimmed {
                                    Ok(DefaultUser)
                                } else {
                                    println!("Invalid credentials");
                                    Err(AuthenticationError::ImplPropagated(
                                        "Invalid credentials".to_string(),
                                        None,
                                    ))
                                }
                            } else {
                                println!(
                                    "Error on give administrator permission: {}",
                                    output.status
                                );
                                Err(AuthenticationError::ImplPropagated(
                                    "Invalid credentials".to_string(),
                                    None,
                                ))
                            }
                        } else {
                            println!("Error on give administrator permission: {}", output.status);
                            Err(AuthenticationError::ImplPropagated(
                                "Invalid credentials".to_string(),
                                None,
                            ))
                        }
                    }
                    Err(e) => {
                        println!("Error on give administrator permission: {}", e);
                        Err(AuthenticationError::ImplPropagated(
                            "Invalid credentials".to_string(),
                            None,
                        )) // Return Err(AuthenticationError::ImplPropagated("Invalid credentials".to_string(), None)) in case of error
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let ftp_home = env::temp_dir();
    let server = Server::with_fs(ftp_home)
        .greeting("welcome to my FTP server!")
        .passive_ports(50000..65535)
        .authenticator(Arc::new(PublicKeyAuthenticator));
    let _ = server.listen("127.0.0.1:2121").await;
}
