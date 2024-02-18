use ludaw::Track;
use rodio::{OutputStream, Sink};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(Track::new()?);
    sink.sleep_until_end();

    Ok(())
}
