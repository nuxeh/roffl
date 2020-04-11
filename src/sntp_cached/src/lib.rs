extern crate failure;
extern crate sntp_request;

use failure::Error;
use std::time::Instant;
use sntp_request::{SntpRequest, SntpTimestamp};

/// Cached NTP struct
pub struct SntpCached {
    timestamp: Option<SntpTimestamp>,
    last_sync: Option<Instant>,
}

impl SntpCached {
    /// Get a new cached NTP struct
    pub fn new() -> Self {
        SntpCached {
            timestamp: None,
            last_sync: None,
        }
    }

    /// Trigger an NTP sync
    pub fn sync(&mut self) -> Result<(), Error> {
        self.sntp_sync()
    }

    /// Get an NTP timestamp for now
    pub fn get_timestamp(self) -> SntpTimestamp {
        unimplemented!();
    }

    /// Get UNIX time as an i64 (seconds)
    pub fn get_unix_time(self) -> i64 {
        42
    }

    /// Sync NTP timestamp
    fn sntp_sync(&mut self) -> Result<(), Error> {
        let sntp = SntpRequest::new();
        self.timestamp = Some(sntp.get_raw_time()?);
        self.last_sync = Some(Instant::now());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
