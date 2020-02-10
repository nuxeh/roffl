enum CommandType {
    Identify,
    Authenticate,
    Notice,
    Message,
    ServerMessage,
}


//#[derive(Serialize, Deserialize)]
struct RofflMessage<'a> {
    command: CommandType,
    content: &'a str
}
