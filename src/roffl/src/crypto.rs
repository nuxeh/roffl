/// Derive a key from passphrase using BLAKE3 as a PBKDF.
pub fn derive_key(passphrase: &str, length: usize) -> String {
    unimplemented!()
}

/// Hash user password using Argon2 hash function.
pub fn hash_user_password(password: &str) -> String {
    unimplemented!()
}

/// Verify user password using Argon2 hash.
pub fn verify_user_password(hash: &str, password: &str) -> bool {
    unimplemented!()
}

/// Encrypt a string
/// Using random key, encrypted with user password as a key

/// Encrypt a block of data
