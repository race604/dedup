use anyhow::{Context, Result};
use clap::Parser;
use log::debug;
use std::{collections::HashSet, fs};

use dedup::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    debug!("{:?}", &args);
    let content = match &args.input {
        Some(path) => fs::read_to_string(path)
            .with_context(|| format!("Failed to read file `{}`", path.display()))?,
        None => return Err(anyhow::anyhow!("No input file specified")),
    };

    debug!("Content of input:\n{}", content);

    let mut cache = HashSet::new();
    for line in content.lines() {
        if cache.contains(line) {
            continue;
        }
        println!("{}", line);
        cache.insert(line);
    }

    Ok(())
}
