mod audio_setup;
mod kp_sound;
mod metronome_core;
mod oscillator;
extern crate clap;

use audio_setup::{host_device_setup, make_strem};
use cpal::traits::StreamTrait;

fn main() -> Result<(), String> {
    let (device, config, format) = host_device_setup();

    let stream = match make_strem(&device, &config, &format) {
        Ok(stream) => stream,
        Err(e) => {
            println!("error {}", e);
            return Err("Stream not built".to_string());
        }
    };

    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));
    Ok(())
}
