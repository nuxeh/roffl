use lazy_static::lazy_static;

use snow::Builder;
use snow::params::NoiseParams;

use crate::crypto::derive_key;

lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XXpsk3_25519_ChaChaPoly_BLAKE2s".parse().unwrap();
}

#[derive(Clone, Copy)]
pub struct Transport {
    secret: [u8; 32],
}

impl Transport {
    /// Connect or begin listening
    fn init() {

    }
}

pub struct TransportBuilder {
    transport: Transport,
}

impl TransportBuilder {
    fn new() -> TransportBuilder {
        let transport = Transport {
            secret: [0; 32],
        };

        TransportBuilder {
            transport,
        }
    }

    fn password(&mut self, password: &str) -> &mut TransportBuilder {
        let secret = derive_key(password, 32);

        for (i, v) in secret.iter().enumerate() {
            self.transport.secret[i] = *v;
        }

        self
    }

    fn build(&self) -> Transport {
        self.transport
    }
}
