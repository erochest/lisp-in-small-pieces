use std::{path::PathBuf, fs::File};

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use env_logger;
use human_panic::setup_panic;

use lisp::error::Result;
use lisp::reader::read_lisp;

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

    match args.command {
        Command::Parse { input } => {
            let mut reader = File::open(input)?;
            let tokens = read_lisp(&mut reader)?;
            for token in tokens {
                println!("{}", serde_json::to_string(&token)?);
            }
        },
    };

    Ok(())
}

/// A small lisp implementation.
/// 
/// I made this while working through the book *Lisp in Small Pieces*.
#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Parse a file and print out the tokens.
    Parse {
        /// The input file to read.
        #[arg(short, long)]
        input: PathBuf,
    },
}
