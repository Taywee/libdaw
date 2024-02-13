use libdaw::{Node, SquareOscillator};
use mlua::prelude::*;
use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

#[derive(Debug, Default)]
struct LibDawSquare(SquareOscillator);

impl Source for LibDawSquare {
    fn current_frame_len(&self) -> Option<usize> {
        Some(1)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for LibDawSquare {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = Vec::new();
        self.0.update(&[&[]], &mut output);
        Some(*output.first().unwrap().first().unwrap() as f32)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = Lua::new();
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(LibDawSquare::default());
    sink.sleep_until_end();

    Ok(())
}
