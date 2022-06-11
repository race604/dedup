use std::path::PathBuf;

use clap::Parser;

/// CLI program to dedupe lines in a file or input stream.
#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
pub struct Cli {
    /// Input file name
    pub input: Option<PathBuf>,

    /// Output file name
    pub output: Option<PathBuf>,

    /// Whether to print verbose output
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}
