use std::error::Error;

use base64::decode;
use openssl::{hash::MessageDigest, pkey::PKey, rsa::Rsa, sign::Verifier};

/// Verify the signature of a message.
pub fn verify_signature(
    public_key_pem: &str,
    message: &str,
    signature_base64: &str,
) -> Result<bool, Box<dyn Error>> {
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes())?;
    let pkey = PKey::from_rsa(rsa)?;

    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey)?;

    verifier.update(message.as_bytes())?;

    let signature = decode(signature_base64)?;

    let is_valid = verifier.verify(&signature)?;

    Ok(is_valid)
}
