pub enum UserEvent<'a> {
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

// TIME?
