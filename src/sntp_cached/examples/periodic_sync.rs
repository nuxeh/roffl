// Periodically sync the internal clock using a thread

use sntp_cached::{SntpCached, rescale_frac_ms};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let read_delay = time::Duration::from_millis(1e+3 as u64);
    let sync_delay = time::Duration::from_millis(10e+3 as u64);

    let ntp = Arc::new(Mutex::new(SntpCached::new()));
    {
        let mut ntp = ntp.lock().unwrap();
        ntp.init().unwrap();
    }

    // Spawn a thread to sync the timestamp periodically
    let ntp_sync = Arc::clone(&ntp);
    let _sync_thread = thread::spawn(move || {
        loop {
            thread::sleep(sync_delay);
            println!("syncing SNTP timestamp");
            {
                let mut ntp = ntp_sync.lock().unwrap();
                ntp.sync().unwrap();
            }
        }
    });

    loop {
        {
            let mut ntp = ntp.lock().unwrap();
            let timestamp = ntp.get_timestamp().unwrap();
            let msec = rescale_frac_ms(&timestamp);
            println!("secs {} frac {} ms {}", timestamp.secs, timestamp.frac, msec);
        }
        thread::sleep(read_delay);
    }
}
