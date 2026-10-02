//! IPN (Instant Payment Notification) verification utilities
//! Matches official docs: sort keys recursively, then HMAC-SHA512

use hmac::{Hmac, KeyInit, Mac};
use serde_json::Value;
use sha2::Sha512;

type HmacSha512 = Hmac<Sha512>;

/// Recursively sort object keys (matches NOWPayments IPN spec)
fn sort_object(obj: &serde_json::Map<String, Value>) -> serde_json::Map<String, Value> {
    let mut sorted = serde_json::Map::new();
    let mut keys: Vec<_> = obj.keys().collect();
    keys.sort();
    for key in keys {
        let val = &obj[key];
        let sorted_val = match val {
            Value::Object(m) => Value::Object(sort_object(m)),
            other => other.clone(),
        };
        sorted.insert(key.clone(), sorted_val);
    }
    sorted
}

/// Verify IPN callback signature from NOWPayments.
/// Safe to call – handles invalid input gracefully.
///
/// # Arguments
/// * `payload` - Raw request body (string or parsed JSON)
/// * `signature` - Value from x-nowpayments-sig header
/// * `ipn_secret` - Your IPN Secret from Dashboard → Store Settings
///
/// # Returns
/// `true` if signature is valid, `false` otherwise
pub fn verify_ipn_signature(
    payload: &str,
    signature: &str,
    ipn_secret: &str,
) -> bool {
    if signature.trim().is_empty() || ipn_secret.trim().is_empty() {
        return false;
    }

    let obj: Value = match serde_json::from_str(payload) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let obj = match obj {
        Value::Object(m) => m,
        _ => return false,
    };

    let sorted = sort_object(&obj);
    let json_string = serde_json::to_string(&Value::Object(sorted)).unwrap_or_default();

    let mut mac = match HmacSha512::new_from_slice(ipn_secret.trim().as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(json_string.as_bytes());
    let result = mac.finalize();
    let computed_bytes = result.into_bytes();

    let sig_hex: String = {
        let mut s = signature.trim().to_lowercase();
        if let Some(idx) = s.rfind('=') {
            s = s[idx + 1..].to_string();
        }
        s.chars()
            .filter(|c| c.is_ascii_hexdigit())
            .collect::<String>()
            .to_lowercase()
    };
    let sig_bytes = match hex::decode(&sig_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };

    if sig_bytes.len() != computed_bytes.len() {
        return false;
    }

    use subtle::ConstantTimeEq;
    AsRef::<[u8]>::as_ref(&computed_bytes)
        .ct_eq(sig_bytes.as_slice())
        .into()
}

/// Create IPN signature for testing (e.g., mocking callbacks)
pub fn create_ipn_signature(payload: &serde_json::Value, ipn_secret: &str) -> Option<String> {
    let obj = payload.as_object()?;
    let sorted = sort_object(obj);
    let json_string = serde_json::to_string(&Value::Object(sorted)).ok()?;

    let mut mac = HmacSha512::new_from_slice(ipn_secret.trim().as_bytes()).ok()?;
    mac.update(json_string.as_bytes());
    let result = mac.finalize();
    Some(hex::encode(result.into_bytes()))
}
