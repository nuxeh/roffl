use sntp_cached::SntpCached;
use std::{thread, time};

fn main() {
    let mut ntp = SntpCached::new();
    ntp.init().unwrap();
    let delay = time::Duration::from_millis(1000);

    loop {
        let timestamp = ntp.get_timestamp().unwrap();
        println!("secs {} frac {}", timestamp.secs, timestamp.frac);
        thread::sleep(delay);
    }
}
