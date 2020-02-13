/// Derive a key from passphrase using BLAKE3 as a PBKDF.
///
/// This is used for example as a PSK for noise protocol communications, the
/// password used here being the "server password".
pub fn derive_key(passphrase: &str, length: usize) -> String {
    unimplemented!()
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
