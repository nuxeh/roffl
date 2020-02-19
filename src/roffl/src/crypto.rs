const SERVER_PASS_CONTEXT: &str = "roffl server pass v1 Wed 19 Feb 22:31:18 CST 2020";

/// Derive a key from passphrase using BLAKE3 as a PBKDF.
///
/// This is used for example as a PSK for noise protocol communications, the
/// password used here being the "server password".
pub fn derive_key(passphrase: &str, bits: usize) -> String {
    let mut output = vec![0; bits / 8];
    blake3::derive_key(SERVER_PASS_CONTEXT, passphrase.as_bytes(), &mut output);
    base64::encode(&output)
}

/// Hash user password using Argon2 hash function.
///
/// Used for authenticating users.
pub fn hash_user_password(password: &str) -> String {
    unimplemented!()
}

/// Verify user password using Argon2 hash.
///
/// Used for authenticating users.
pub fn verify_user_password(hash: &str, password: &str) -> bool {
    unimplemented!()
}

/// Encrypt a string.
///
/// Using random key, encrypted with user password as a key.
fn encrypt_string(key: &str, string: &str) -> String {
    unimplemented!()
}

/// Encrypt a block of data.
fn encrypt_block(key: &str, block: &[u8]) -> Vec<u8> {
    unimplemented!()
}

/// Generate a hopefully cryptographically secure random encryption key.
fn generate_random_key() -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key() {
        assert_eq!(derive_key("hunter2", 256), "xGc2M/5ZA5BwL9ZpZ1TXp5VODBh4/oU98tmyWym3a3k=");
    }
}
