use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use minigrep::search;

#[derive(Parser)]
#[command(about = "A minimal grep clone")]
struct Args {
    /// Search pattern
    pattern: String,

    /// File path to search
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let contents = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file: {}", args.path.display()))?;

    for (line_num, line) in search(&args.pattern, &contents) {
        println!("{line_num}:{line}");
    }

    Ok(())
}
