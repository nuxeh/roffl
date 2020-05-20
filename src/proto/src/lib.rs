#[macro_use] extern crate serde_derive;

pub mod types;
pub mod timestamp;
pub mod event;

pub enum MessageType<'a> {
    Identify(Identify<'a>),
    Authenticate(Authenticate<'a>),
    Notice(Notice<'a>),
    Message(Message<'a>),
    ServerMessage(ServerMessage<'a>),
    Command(Command<'a>),
    Backscroll(Backscroll<'a>),
    Ping,
    PingResponse,
    Keepalive,
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

pub enum CommandType {
    Mode,
    Oper,
}

pub struct Command<'a> {
    kind: CommandType,
    params: Vec<&'a str>,
}

pub struct Backscroll<'a> {
    something: &'a str,
}
