use anyhow::{Context, Result};
use clap::Parser;
use log::debug;
use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

use dedup::cache::Cache;
use dedup::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    debug!("{:?}", &args);
    let input: Box<dyn BufRead> = match &args.input {
        Some(path) => {
            let file = File::open(path)
                .with_context(|| format!("Failed to open file `{}`", path.display()))?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(stdin().lock())),
    };

    let mut output: Box<dyn Write> = match &args.output {
        Some(path) => {
            let file = File::create(path)
                .with_context(|| format!("Failed to create file `{}`", path.display()))?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(stdout().lock()),
    };

    let mut cache = Cache::new();
    for line in input.lines().into_iter() {
        let line = line.with_context(|| "Failed to read line")?;
        if cache.contains(&line) {
            continue;
        }
        output
            .write_fmt(format_args!("{}\n", &line))
            .with_context(|| "Failed to write line")?;
        cache.insert(line);
    }

    Ok(())
}
