use rand_core::{RngCore, OsRng};
use base64::URL_SAFE;

/// Default number of bytes of entropy to generate
const BYTES: usize = 20; // 160 bits

/// Generate a cryptographically secure session ID with
/// a specified amount of entropy in bytes, encoded as base64
pub fn generate_bits(bytes: usize) -> String {
    let mut buffer = vec![0u8; bytes];
    OsRng.fill_bytes(&mut buffer);
    base64::encode_config(&buffer, URL_SAFE)
}

/// Generate a cryptographically secure session ID with
/// default amount of entropy, encoded as base64
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
