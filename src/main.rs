use std::{path::PathBuf, fs::File};

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use env_logger;
use human_panic::setup_panic;

use lisp::error::Result;
use lisp::reader::read_lisp;

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
