use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();

    grrs::find_matches("cat\n", "cat", &mut result);
    assert_eq!(result, b"cat\n\n");
}


fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);

    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file \"{}\"", &args.path.to_string_lossy()))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let content = line?;
        grrs::find_matches(&content, &args.pattern, &mut writer)?;
    }

    Ok(())
}
