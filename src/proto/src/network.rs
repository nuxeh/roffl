#[derive(Serialize, Deserialize)]
enum NetworkType {
    Irc,
    Rc,
};

#[derive(Serialize, Deserialize)]
struct Network {
    nick: &str,
    type: NetworkType,
};

#[derive(Serialize, Deserialize)]
struct NetworkBlock {
    networks: Vec<Network>,
};
