use lazy_static::lazy_static;

use snow::Builder;
use snow::params::NoiseParams;

use crate::crypto::derive_key;

lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XXpsk3_25519_ChaChaPoly_BLAKE2s".parse().unwrap();
}

pub struct Connector {
    secret: [u8; 32],
}

impl Connector {
    /// Connect or begin listening
    fn init() {

    }
}

pub struct TransportBuilder {
    secret: [u8; 32],
}

impl TransportBuilder {
    fn new() -> TransportBuilder {
        TransportBuilder {
            secret: [0; 32],
        }
    }

    fn password(&mut self, password: &str) -> &mut TransportBuilder {
        let secret = derive_key(password, 32);

        for (i, v) in secret.iter().enumerate() {
            self.secret[i] = *v;
        }

        self
    }

    fn build(&self) -> Connector {
        Connector {
            secret: self.secret,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_password() {
        let t = TransportBuilder::new()
            .password("foobarbaz")
            .build();

        assert_eq!(
            base64::encode(&t.secret),
            "IG/rVwGaJn3JUrVtAymJw2XNA9XN8BIPYpk/thUnZVk="
        );
        assert_eq!(
            base64::encode(&t.secret),
            base64::encode(&derive_key("foobarbaz", 32))
        );
    }
}
