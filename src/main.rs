use anyhow::{Context, Result};
use clap::Parser;
use log::debug;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use dedup::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    debug!("{:?}", &args);
    let file = match &args.input {
        Some(path) => {
            File::open(path).with_context(|| format!("Failed to open file `{}`", path.display()))?
        }
        None => return Err(anyhow::anyhow!("No input file specified")),
    };

    let reader = BufReader::new(file);
    let mut cache = HashSet::new();
    for line in reader.lines().into_iter() {
        let line = line.with_context(|| "Failed to read line")?;
        if cache.contains(&line) {
            continue;
        }
        println!("{}", line);
        cache.insert(line);
    }

    Ok(())
}
