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
        println!("FTP Home: {:?}", ftp_home);

        let public_key_path = ftp_home.join(format!("{}.pub", username));

        let result = std::fs::read_to_string(&public_key_path);
        match result {
            Ok(public_key_content) => Ok(DefaultUser),

            Err(_) => {
                let output = Command::new("runas")
                    .arg("/user:Administrator")
                    .arg("cmd")
                    .arg("/c")
                    .arg("type")
                    .arg(&public_key_path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();

                match output {
                    Ok(output) => {
                        if output.status.success() {
                            // Ahora podemos leer la salida del proceso de ejecución
                            if let Ok(output_str) = String::from_utf8(output.stdout) {
                                println!("Contenido del archivo: {}", output_str);
                                // Haz lo que necesites con el contenido del archivo
                            }
                            Ok(DefaultUser)
                        } else {
                            Err(AuthenticationError::ImplPropagated(
                                "Invalid credentials".to_string(),
                                None,
                            ))
                        }
                    }
                    Err(e) => {
                        println!("Error al solicitar permisos de administrador: {}", e);
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
        .greeting("¡Bienvenido a mi servidor FTP!")
        .passive_ports(50000..65535)
        .authenticator(Arc::new(PublicKeyAuthenticator));
    let _ = server.listen("127.0.0.1:2121").await;
}
