use lazy_static::lazy_static;

use snow::Builder;
use snow::params::NoiseParams;

use crate::crypto::derive_key;

lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XXpsk3_25519_ChaChaPoly_BLAKE2s".parse().unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConnectionType {
    Server,
    Client,
}

pub struct Connection {
    kind: ConnectionType,
    secret: [u8; 32],
}

impl Connection {
    /// Connect or begin listening
    pub fn init(&self) {
        if self.kind == ConnectionType::Client {
            self.init_client();
        } else {
            self.init_server();
        }
    }

    fn init_client(&self) {

    }

    fn init_server(&self) {

    }
}

pub struct ConnectionBuilder {
    connector: Connection,
}

impl ConnectionBuilder {
    pub fn client() -> ConnectionBuilder {
        ConnectionBuilder {
            connector: Connection {
                kind: ConnectionType::Client,
                secret: [0; 32],
            },
        }
    }

    pub fn server() -> ConnectionBuilder {
        ConnectionBuilder {
            connector: Connection {
                kind: ConnectionType::Server,
                secret: [0; 32],
            },
        }
    }

    pub fn password(&mut self, password: &str) -> &mut ConnectionBuilder {
        let secret = derive_key(password, 32);

        for (i, v) in secret.iter().enumerate() {
            self.connector.secret[i] = *v;
        }

        self
    }

    pub fn build(&self) -> Connection {
        Connection { ..self.connector }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_client() {
        let t = ConnectionBuilder::client().build();
        assert_eq!(t.kind, ConnectionType::Client);
        assert_ne!(t.kind, ConnectionType::Server);
    }

    #[test]
    fn test_build_server() {
        let t = ConnectionBuilder::server().build();
        assert_eq!(t.kind, ConnectionType::Server);
        assert_ne!(t.kind, ConnectionType::Client);
    }

    #[test]
    fn test_set_password() {
        let t = ConnectionBuilder::client()
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
