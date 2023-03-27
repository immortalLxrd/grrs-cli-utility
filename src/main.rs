use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};
use std::{fs::File, io::BufRead, io::BufReader, path::PathBuf};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file \"{}\"", &args.path.to_string_lossy()))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let content = line?;
        if content.contains(&args.pattern) {
            writeln!(handle, "{}", content)?;
        }
    }

    Ok(())
}
