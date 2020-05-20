use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
rcchat bridge (server).

Usage:
    rcchat_bridge
    rcchat_bridge --help
    rcchat_bridge --version

Options:
    -h --help     Show this screen.
    --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
}


fn main() {
    // Parse CLI args
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(String::from("0.1.0"))).parse())
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Hello, world!");
}
