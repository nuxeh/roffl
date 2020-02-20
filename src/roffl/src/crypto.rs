use rand::Rng;

const USER_PASS_SALT_SIZE: usize = 32;
const SERVER_PASS_CONTEXT: &str = "roffl server pass v1 Wed 19 Feb 22:31:18 CST 2020";

/// Derive a key from passphrase using BLAKE3 as a PBKDF.
///
/// This is used for example as a PSK for noise protocol communications, the
/// password used here being the "server password".
pub fn derive_key(passphrase: &str, bits: usize) -> Vec<u8> {
    let mut output = vec![0; bits / 8];
    blake3::derive_key(SERVER_PASS_CONTEXT, passphrase.as_bytes(), &mut output);
    output
}

pub struct HashedPassword {
    hash: String,
    salt: Vec<u8>,
}

/// Hash user password
///
/// Used for authenticating users. Returns a 2 array of strings, first salt,
/// then hash, both Base64 encoded.
pub fn hash_user_password(pass: &str) -> HashedPassword {
    let salt = generate_random_data(USER_PASS_SALT_SIZE);

    HashedPassword {
        hash: hash_password(pass, &salt),
        salt,
    }
}

/// Hash a password using Argon2 hash function.
///
/// Hashed password is returned in a Base64 encoded string.
fn hash_password(pass: &str, salt: &[u8]) -> String {
    let conf = argon2::Config::default();
    argon2::hash_encoded(pass.as_bytes(), salt, &conf).unwrap()
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

/// Generate a cryptographically secure random encryption key.
fn generate_random_data(bytes: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let output: Vec<u8> = vec![0; bytes];
    let output: Vec<u8> = output
        .iter()
        .map(|_| rng.gen::<u8>())
        .collect();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key() {
        let derived = base64::encode(&derive_key("hunter2", 256));
        assert_eq!(derived, "xGc2M/5ZA5BwL9ZpZ1TXp5VODBh4/oU98tmyWym3a3k=");
    }

    #[test]
    fn test_generate_random_data() {
        assert_eq!(generate_random_data(8).len(), 8);
        assert_eq!(generate_random_data(128).len(), 128);
        let generated = base64::encode(&generate_random_data(8));
        assert_ne!(generated, base64::encode(&[0; 8]));
        assert_ne!(generated, base64::encode(&[255; 8]));
    }

    #[test]
    fn test_hash_password() {
        assert_eq!(
            hash_password("hunter2", b"saltysaltysalty"),
            "$argon2i$v=19$m=4096,t=3,p=1$c2FsdHlzYWx0eXNhbHR5$68Gy4/yqHHgRxiN/YDPMfB2X+JAqw47VmER7obMQfQY"
        );
    }
}
