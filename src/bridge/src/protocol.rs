pub mod irc;

pub trait Protocol {
    fn connect();
    fn disconnect();
}
