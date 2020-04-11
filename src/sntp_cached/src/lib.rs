extern crate sntp_request;

use std::io;
use std::time::Instant;
use sntp_request::SntpTimestamp;

pub struct SntpCached {
    timestamp: Option<SntpTimestamp>,
    last_sync: Option<Instant>,
}

impl SntpCached {
    pub fn sync() -> io::Result<()> {
        Ok(())
    }

    pub fn get_timestamp() -> SntpTimestamp {
        unimplemented!();
    }

    pub fn get_unix_time() -> i64 {
        42
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
