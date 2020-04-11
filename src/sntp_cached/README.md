# `sntp_cached`

A small Rust library to maintain a cache of NTP time, updating the cached NTP
time intermittently, while maintaining an internal clock to provide timestamps,
without repeatedly querying NTP.
