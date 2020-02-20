use lazy_static::lazy_static;

use snow::Builder;
use snow::params::NoiseParams;

use crate::crypto::derive_key;

lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XXpsk3_25519_ChaChaPoly_BLAKE2s".parse().unwrap();
}

#[derive(Clone, Copy)]
enum ConnectorType {
    Server,
    Client,
}

pub struct Connector {
    kind: ConnectorType,
    secret: [u8; 32],
}

impl Connector {
    /// Connect or begin listening
    fn init() {

    }
}

pub struct TransportBuilder {
    connector: Connector,
}

impl TransportBuilder {
    fn client() -> TransportBuilder {
        TransportBuilder {
            connector: Connector {
                kind: ConnectorType::Client,
                secret: [0; 32],
            },
        }
    }

    fn server() -> TransportBuilder {
        TransportBuilder {
            connector: Connector {
                kind: ConnectorType::Server,
                secret: [0; 32],
            },
        }
    }

    fn password(&mut self, password: &str) -> &mut TransportBuilder {
        let secret = derive_key(password, 32);

        for (i, v) in secret.iter().enumerate() {
            self.connector.secret[i] = *v;
        }

        self
    }

    fn build(&self) -> Connector {
        Connector { ..self.connector }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_password() {
        let t = TransportBuilder::client()
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
