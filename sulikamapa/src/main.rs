mod transpile;
use clap::Parser;
mod html;

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::transpile::transpile_stream;

#[derive(Parser)]
struct Cli {
    /// Input file (or stdin if not specified)
    #[arg(short, long)]
    input: Option<String>,

    /// Output file (or stdout if not specified)
    #[arg(short, long)]
    output: Option<String>,
}

fn get_input(path: Option<&str>) -> Result<Box<dyn Read>, io::Error> {
    match path {
        Some(path) => Ok(Box::new(BufReader::new(File::open(path)?))),
        None => Ok(Box::new(io::stdin().lock())),
    }
}

fn get_output(path: Option<&str>) -> Result<Box<dyn Write>, io::Error> {
    match path {
        Some(path) => Ok(Box::new(BufWriter::new(File::create(path)?))),
        None => Ok(Box::new(io::stdout().lock())),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut input = get_input(cli.input.as_deref())?;
    let mut output = get_output(cli.output.as_deref())?;

    transpile_stream(&mut *input, &mut *output)?;

    Ok(())
}
