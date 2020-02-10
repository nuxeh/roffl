pub enum CommandType {
    Identify(Identify),
    Authenticate(Authenticate),
    Notice(Notice),
    Message(Message),
    ServerMessage(ServerMessage),
    Backscroll(Backscroll),
}

//#[derive(Serialize, Deserialize)]
pub struct Identify<'a> {
    name: &'a str,
}

pub struct Authenticate<'a> {
    password: &'a str,
}

pub struct Notice<'a> {
    content: &'a str,
}

pub struct Message<'a> {
    content: &'a str,
}

pub struct ServerMessage<'a> {
    content: &'a str,
}

pub struct Ping<'a> {
    content: &'a str,
}

pub struct Backscroll<'a> {
    content: &'a str,
}
