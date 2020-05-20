use chrono::{DateTime, Local, Utc};

#[derive(Default, Clone, Copy, Debug)]
pub struct Timestamp {
    epoch: u32,
    timestamp: u64,
    lamport: u32,
}

//impl Display for Timestamp {
//}

impl Timestamp {
    /// Get a `DateTime` representing the local time of the Timestamp.
    pub fn to_date_time_utc() -> DateTime<Utc> {
        unimplemented!()
    }

    /// Get a `DateTime` representing the local time of the Timestamp.
    pub fn to_date_time_local() -> DateTime<Local> {
        unimplemented!()
    }
}
