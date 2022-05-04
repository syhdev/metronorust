mod audio_setup;
mod kp_sound;
mod metronome_core;
use clap::Parser;

use audio_setup::{host_device_setup, make_strem};
use cpal::traits::StreamTrait;

#[derive(Parser)]
#[clap(name = "Metronorust")]
#[clap(author = "syh.dev")]
#[clap(version = "0.1")]
#[clap(about = "A simple metronome", long_about = None)]
struct Cli {
    #[clap(short, long)]
    bpm: usize,
    #[clap(short, long)]
    time_signature: usize,
    #[clap(short, long, default_value_t = 1)]
    subdiv: usize,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let (device, config, format) = host_device_setup();

    let stream = match make_strem(
        &device,
        &config,
        &format,
        cli.bpm,
        cli.time_signature,
        cli.subdiv,
    ) {
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
