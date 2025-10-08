use hmac::{Hmac, Mac};
use rsa::{
    RsaPublicKey,
    pkcs1::DecodeRsaPublicKey,
    pkcs1v15::{Signature, VerifyingKey},
    pkcs8::DecodePublicKey,
    signature::Verifier,
};
use serde_json::Value;
use sha2::{Sha256, Sha384, Sha512};
use zed_extension_api::Result;

pub fn verify(header: &Value, signing_input: &str, sig: &[u8], key: &str) -> Result<bool, String> {
    let alg = header
        .get("alg")
        .and_then(|v| v.as_str())
        .ok_or("missing alg in header".to_string())?;

    match alg {
        "HS256" => verify_hmac_sha256(signing_input, sig, key.as_bytes()),
        "HS384" => verify_hmac_sha384(signing_input, sig, key.as_bytes()),
        "HS512" => verify_hmac_sha512(signing_input, sig, key.as_bytes()),
        "RS256" => verify_rsa_sha256(signing_input, sig, key),
        "RS384" => verify_rsa_sha384(signing_input, sig, key),
        "RS512" => verify_rsa_sha512(signing_input, sig, key),
        "none" => Ok(sig.is_empty()),
        _ => Err(format!("unsupported algorithm: {}", alg)),
    }
}

fn verify_hmac_sha256(input: &str, sig: &[u8], key: &[u8]) -> Result<bool, String> {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(key).map_err(|_| "invalid key length".to_string())?;
    mac.update(input.as_bytes());
    Ok(mac.verify_slice(sig).is_ok())
}

fn verify_hmac_sha384(input: &str, sig: &[u8], key: &[u8]) -> Result<bool, String> {
    let mut mac =
        Hmac::<Sha384>::new_from_slice(key).map_err(|_| "invalid key length".to_string())?;
    mac.update(input.as_bytes());
    Ok(mac.verify_slice(sig).is_ok())
}

fn verify_hmac_sha512(input: &str, sig: &[u8], key: &[u8]) -> Result<bool, String> {
    let mut mac =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| "invalid key length".to_string())?;
    mac.update(input.as_bytes());
    Ok(mac.verify_slice(sig).is_ok())
}

fn verify_rsa_sha256(input: &str, sig: &[u8], key: &str) -> Result<bool, String> {
    let public_key = RsaPublicKey::from_pkcs1_pem(key)
        .or_else(|_| RsaPublicKey::from_public_key_pem(key))
        .map_err(|e| format!("failed to parse public key: {}", e))?;

    let verifying_key: VerifyingKey<Sha256> = VerifyingKey::new(public_key);

    let signature =
        Signature::try_from(sig).map_err(|e| format!("invalid signature length: {}", e))?;

    Ok(verifying_key.verify(input.as_bytes(), &signature).is_ok())
}

fn verify_rsa_sha384(input: &str, sig: &[u8], key: &str) -> Result<bool, String> {
    let public_key = RsaPublicKey::from_pkcs1_pem(key)
        .or_else(|_| RsaPublicKey::from_public_key_pem(key))
        .map_err(|e| format!("failed to parse public key: {}", e))?;

    let verifying_key: VerifyingKey<Sha384> = VerifyingKey::new(public_key);

    let signature =
        Signature::try_from(sig).map_err(|e| format!("invalid signature length: {}", e))?;

    Ok(verifying_key.verify(input.as_bytes(), &signature).is_ok())
}

fn verify_rsa_sha512(input: &str, sig: &[u8], key: &str) -> Result<bool, String> {
    let public_key = RsaPublicKey::from_pkcs1_pem(key)
        .or_else(|_| RsaPublicKey::from_public_key_pem(key))
        .map_err(|e| format!("failed to parse public key: {}", e))?;

    let verifying_key: VerifyingKey<Sha512> = VerifyingKey::new(public_key);

    let signature =
        Signature::try_from(sig).map_err(|e| format!("invalid signature length: {}", e))?;

    Ok(verifying_key.verify(input.as_bytes(), &signature).is_ok())
}
