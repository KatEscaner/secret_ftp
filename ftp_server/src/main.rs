use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials, DefaultUser};
use libunftp::Server;
use openssl::x509::verify;
use std::env;
use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};
use std::sync::Arc;
use unftp_sbe_fs::ServerExt;

#[derive(Debug)]
struct PublicKeyAuthenticator;

mod utils;
use crate::utils::{fs_utils, openssl_utils};

#[async_trait]
impl Authenticator<DefaultUser> for PublicKeyAuthenticator {
    async fn authenticate(
        &self,
        username: &str,
        password: &Credentials,
    ) -> Result<DefaultUser, AuthenticationError> {
        match fs_utils::get_public_key(username) {
            Ok(public_key) => {
                match openssl_utils::verify_signature(
                    &public_key,
                    username,
                    password.password.to_owned().unwrap().as_str(),
                ) {
                    Ok(is_valid) => {
                        if is_valid {
                            return Ok(DefaultUser);
                        }
                    }
                    Err(e) => {
                        println!("Error on verify signature: {}", e);
                        return Err(AuthenticationError::BadPassword);
                    }
                }
            }
            Err(e) => {
                println!("Error on get public key: {}", e);
                return Err(AuthenticationError::BadPassword);
            }
        }
        Err(AuthenticationError::BadPassword)
    }
}

#[tokio::main]
async fn main() {
    let ftp_home = env::current_dir().unwrap().join("resources");
    let server: Server<unftp_sbe_fs::Filesystem, DefaultUser> = Server::with_fs(ftp_home)
        .greeting("welcome to my FTP server!")
        .passive_ports(50000..65535)
        .authenticator(Arc::new(PublicKeyAuthenticator))
        .ftps("server.certs", "server.key");
    let _ = server.listen("127.0.0.1:2121").await;
}
