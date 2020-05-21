use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
rcchat make release utility.

Usage:
    make_release --all --bump
    make_release --package FOO --bump
    make_release --all --bump --major
    make_release --package FOO --bump --major
    make_release --help
    make_release --version

Options:
    -h --help       Show this screen.
    --version       Show version.
    --bump          Bump the package version.
    --package=FOO   Which package to bump.
    --major         Make a major release.
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

    /// Put git ref in a file
    /// Commit
    /// Tag
    /// Ensure using latest stable Rust toolchain, and update release.nix
}
