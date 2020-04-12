extern crate failure;
extern crate sntp_request;

use failure::{Error, bail};
use std::time::Instant;
use sntp_request::{SntpRequest, SntpTimestamp};

/// Cached NTP struct
pub struct SntpCached {
    /// The last cached NTP timestamp
    timestamp: Option<SntpTimestamp>,
    /// An instant for counting the time since the update
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

    /// Initialise
    pub fn init(&mut self) -> Result<(), Error> {
        self.sync()
    }

    /// Trigger an NTP sync
    /// TODO: Consider making this asynchronous to cope with long updates
    pub fn sync(&mut self) -> Result<(), Error> {
        self.sntp_sync()
    }

    /// Get an NTP timestamp for now
    pub fn get_timestamp(&mut self) -> Result<SntpTimestamp, Error> {
        if self.timestamp.is_none() {
            self.sync()?
        }

        let nsec = self.last_sync
            .as_ref()
            .map(|last| last.elapsed().as_nanos());

        let ts = if let (Some(t), Some(ns)) = (self.timestamp.as_ref(), nsec) {
            let ntp_ns = (t.frac as f64 / u32::max_value() as f64)*1.0e+9;
            let ns_elapsed = ns + ntp_ns as u128;
            let sec_elapsed = ns_elapsed / 1.0e+9 as u128;
            let ns_remain = ns_elapsed % 1.0e+9 as u128;
            let frac = (ns_remain as f64 / 1.0e+9) * u32::max_value() as f64;

            SntpTimestamp {
                secs: t.secs + sec_elapsed as u32,
                frac: frac as u32,
            }
        } else {
            bail!("NTP clock not initialised");
        };

        Ok(ts)
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

/// Rescale the fractional part of the timestamp to a given scale
pub fn rescale_frac(ts: &SntpTimestamp, scale: u128) -> u128 {
    ((ts.frac as f64 / u32::max_value() as f64) * scale as f64) as u128
}

/// Rescale the fractional part of the timestamp to milliseconds
pub fn rescale_frac_ms(ts: &SntpTimestamp) -> u32 {
    rescale_frac(ts, 1e+3 as u128) as u32
}

/// Rescale the fractional part of the timestamp to nanoseconds
pub fn rescale_frac_ns(ts: &SntpTimestamp) -> u128 {
    rescale_frac(ts, 1e+9 as u128) as u128
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
