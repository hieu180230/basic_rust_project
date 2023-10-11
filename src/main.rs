use std::error::Error;
use rusty_audio::Audio;
fn main() -> Result<(), Box<dyn Error>>{
    let mut audio = Audio::new();
    audio.add("name", "filename.wav");

    audio.play("name"); //this play audio
    audio.wait(); //this wait for audio to finish
    Ok(()) //this return Ok
}
