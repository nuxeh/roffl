use crate::timestamp::Timestamp;

pub struct Event<T> {
    evt: T,
    timestamp: Timestamp,
}

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
