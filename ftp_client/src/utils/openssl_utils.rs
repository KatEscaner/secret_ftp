use base64::encode;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Signer;
use std::error::Error;

pub fn sign_message(private_key_pem: &str, message: &str) -> Result<String, Box<dyn Error>> {
    let rsa = Rsa::private_key_from_pem(private_key_pem.as_bytes())?;
    let pkey = PKey::from_rsa(rsa)?;

    let mut signer = Signer::new(openssl::hash::MessageDigest::sha256(), &pkey)?;

    signer.update(message.as_bytes())?;

    let signature = signer.sign_to_vec()?;

    let signature_base64 = encode(&signature);

    Ok(signature_base64)
}
