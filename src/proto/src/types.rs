use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Namespace {
    pub channels: Vec<Channel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub head: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub child: String,
    //pub messages: Vec<Message>,
    //pub head: Message,
}
