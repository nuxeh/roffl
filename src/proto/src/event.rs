pub enum Event<'a> {
    Message {
        nick: &'a str,
        text: &'a str,
    },
    Notice {
        nick: &'a str,
        text: &'a str,
    },
    Join {
        nick: &'a str,
    },
    Part {
        nick: &'a str,
    }
}
