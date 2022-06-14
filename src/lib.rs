use std::path::PathBuf;

use clap::Parser;

/// CLI program to dedupe lines in a file or input stream.
#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
pub struct Cli {
    /// Input file name
    #[clap(value_parser, value_name = "INPUT FILE")]
    pub input: Option<PathBuf>,

    /// Output file name
    #[clap(value_parser, value_name = "OUTPUT FILE")]
    pub output: Option<PathBuf>,

    /// Memory limit in bytes, 0 for unlimited
    #[clap(short, long, action = clap::ArgAction::Set, default_value_t = 0)]
    pub memo_limit: usize,

    /// Whether to print verbose output
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

pub mod cache;
