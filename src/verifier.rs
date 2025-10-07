use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::{Digest, Sha256, Sha384, Sha512};
use zed_extension_api::Result;

pub fn verify(header: &Value, signing_input: &str, sig: &[u8], key: &str) -> Result<bool, String> {
    let alg = header
        .get("alg")
        .and_then(|v| v.as_str())
        .ok_or("missing alg in header".to_string())?;

    match alg {
        "HS256" => verify_hmac::<Sha256>(signing_input, sig, key.as_bytes()),
        "HS384" => verify_hmac::<Sha384>(signing_input, sig, key.as_bytes()),
        "HS512" => verify_hmac::<Sha512>(signing_input, sig, key.as_bytes()),
        "none" => Ok(sig.is_empty()),
        _ => Err(format!("unsupported algorithm: {}", alg)),
    }
}

fn verify_hmac<D>(input: &str, sig: &[u8], key: &[u8]) -> Result<bool, String>
where
    D: Digest + 'static,
{
    if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Sha256>() {
        verify_hmac_sha256(input, sig, key)
    } else if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Sha384>() {
        verify_hmac_sha384(input, sig, key)
    } else if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Sha512>() {
        verify_hmac_sha512(input, sig, key)
    } else {
        Err("unsupported digest type".to_string())
    }
}

fn verify_hmac_sha256(input: &str, sig: &[u8], key: &[u8]) -> Result<bool, String> {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(key).map_err(|_| "invalid key length".to_string())?;
    mac.update(input.as_bytes());
    let computed = mac.finalize().into_bytes();
    Ok(computed.as_slice() == sig)
}

fn verify_hmac_sha384(input: &str, sig: &[u8], key: &[u8]) -> Result<bool, String> {
    let mut mac =
        Hmac::<Sha384>::new_from_slice(key).map_err(|_| "invalid key length".to_string())?;
    mac.update(input.as_bytes());
    let computed = mac.finalize().into_bytes();
    Ok(computed.as_slice() == sig)
}

fn verify_hmac_sha512(input: &str, sig: &[u8], key: &[u8]) -> Result<bool, String> {
    let mut mac =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| "invalid key length".to_string())?;
    mac.update(input.as_bytes());
    let computed = mac.finalize().into_bytes();
    Ok(computed.as_slice() == sig)
}
