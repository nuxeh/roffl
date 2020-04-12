extern crate failure;
extern crate sntp_request;

use failure::{Error, bail};
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

    pub fn init(&mut self) -> Result<(), Error> {
        self.sync()
    }

    /// Trigger an NTP sync
    pub fn sync(&mut self) -> Result<(), Error> {
        self.sntp_sync()
    }

    /// Get an NTP timestamp for now
    pub fn get_timestamp(&mut self) -> Result<SntpTimestamp, Error> {
        if let None = self.timestamp {
            self.sync()?
        }

        let elapsed = self.last_sync
            .as_ref()
            .and_then(|last| Some(last.elapsed().as_nanos()));

        let timestamp = if let (Some(t), Some(e)) = (self.timestamp.as_ref(), elapsed) {
            let ntp_ns = (t.frac as f64 / u32::max_value() as f64)*1000000000.0;
            let ns_elapsed = e + ntp_ns as u128;
            let sec_elapsed = ns_elapsed / 1000000000;
            let ns_remain = ns_elapsed % 1000000000;
            let frac = (ns_remain as f64 / 1000000000.0) * u32::max_value() as f64;

            SntpTimestamp {
                secs: t.secs + sec_elapsed as u32,
                frac: frac as u32,
            }
        } else {
            bail!("NTP clock not initialised");
        };

        Ok(timestamp)
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
