
use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use env_logger;
use human_panic::setup_panic;

mod error;

use error::Result;

// TODO: read from a file and dump out syntax
// TODO: parse an integer (`42`)
// TODO: read from stdin
// TODO: parse a symbol (`foobar`)
// TODO: parse a float (`3.14159`)
// TODO: parse a rational number (`2/3`)
// TODO: parse a string (`"string with spaces"`)
// TODO: parse nil (`nil`)
// TODO: parse an empty list (`()`)
// TODO: parse a dotted-cons cell (`(42 . 43)`)
// TODO: parse a cons list (`(42 43 44)`)
// TODO: parse a quoted symbol (`'foobar`)
// TODO: parse a quoted list (`'(+ 1 3)`)
// TODO: parse a quoted function name (`#'foobar`)
// TODO: parse comments

fn main() -> Result<()> {
    setup_panic!();
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    println!("Hello, world!");

    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,

    #[arg(short, long, help = "The input file to read.")]
    input: PathBuf,
}