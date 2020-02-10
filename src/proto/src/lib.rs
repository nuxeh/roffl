pub enum CommandType<'a> {
    Identify(Identify<'a>),
    Authenticate(Authenticate<'a>),
    Notice(Notice<'a>),
    Message(Message<'a>),
    ServerMessage(ServerMessage<'a>),
    Backscroll(Backscroll<'a>),
}

//#[derive(Serialize, Deserialize)]
pub struct Identify<'a> {
    name: &'a str,
}

pub struct Authenticate<'a> {
    pass: &'a str,
}

pub struct Notice<'a> {
    text: &'a str,
}

pub struct Message<'a> {
    text: &'a str,
}

pub struct ServerMessage<'a> {
    text: &'a str,
}

pub struct Ping<'a> {
    time: &'a str,
}

pub struct Backscroll<'a> {
    something: &'a str,
}
