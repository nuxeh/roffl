use rand_core::{RngCore, OsRng};
use base64::URL_SAFE;

/// Default number of session ID bytes to generate
const BYTES: usize = 160 / 8;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let first = generate();
        let second = generate();
        assert_ne!(first, second);
        println!("{}", first);
        println!("{}", second);
        assert_eq!(first.len(), 28);
        assert_eq!(second.len(), 28);
    }
}
