// Sync the clock on initialisation, and just use the internal clock from then
// onwards. Timing will creep due to OS scheduling of the sleep.

use sntp_cached::{SntpCached, rescale_frac_ms};
use std::{thread, time};

fn main() {
    let mut ntp = SntpCached::new();
    ntp.init().unwrap();
    let delay = time::Duration::from_millis(1000);

    loop {
        let timestamp = ntp.get_timestamp().unwrap();
        let msec = rescale_frac_ms(&timestamp);
        println!("secs {} frac {} ms {}", timestamp.secs, timestamp.frac, msec);
        thread::sleep(delay);
    }
}
