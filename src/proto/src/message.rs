#[derive(Serialize, Deserialize)]
struct Messagei<'a> {
    sender: &'a str,
    recipient: &'a str,
    message: &'a str,
};

#[derive(Serialize, Deserialize)]
struct MessageBlock {
    messages: Vec<Message>,
    bloom_filter: String,
};
