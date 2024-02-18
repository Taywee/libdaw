use std::path::PathBuf;

use clap::{Args, Parser};
use ludaw::Track;
use rodio::{OutputStream, Sink};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    input: Input,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Input {
    /// Input file
    #[arg(short, long, value_name = "file")]
    input: Option<PathBuf>,

    /// Name of the person to greet
    #[arg(short, long, value_name = "script")]
    execute: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let script = if let Some(path) = cli.input.input {
        std::fs::read_to_string(path)?
    } else if let Some(script) = cli.input.execute {
        script
    } else {
        unreachable!()
    };

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let (mut track, source) = Track::new(script)?;
    sink.append(source);
    loop {
        track.process()?;
    }

    Ok(())
}
