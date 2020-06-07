use rand_core::{RngCore, OsRng};
use base64::URL_SAFE;

/// Default number of session ID bytes to generate
const BYTES: usize = 160;

/// Generate a cryptographically secure session ID with
/// size of a specified number of bytes, encoded as base64
pub fn generate_bits(bytes: usize) -> String {
    let mut buffer = vec![0u8; bytes];
    OsRng.fill_bytes(&mut buffer);
    base64::encode_config(&buffer, URL_SAFE)
}

/// Generate a cryptographically secure session ID with
/// default size, encoded as base64
pub fn generate() -> String {
    generate_bits(BYTES)
}
