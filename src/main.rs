mod audio_setup;
mod kp_sound;
mod metronome_core;
use clap::Parser;

use audio_setup::{host_device_setup, make_strem};
use cpal::traits::StreamTrait;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::pixels::Color;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[derive(Parser)]
#[clap(name = "Metronorust")]
#[clap(author = "syh.dev")]
#[clap(version = "0.1")]
#[clap(about = "A simple metronome", long_about = None)]
struct Cli {
    #[clap(short, long, default_value_t = 60)]
    bpm: usize,
    #[clap(short, long, default_value_t = 4)]
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
    std::thread::sleep(std::time::Duration::from_millis(3000));

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut lastx = 0;
    let mut lasty = 0;

    let mut events = sdl_context.event_pump()?;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else if keycode == Keycode::Space {
                        println!("space down");
                        for i in 0..400 {
                            canvas.pixel(i as i16, i as i16, 0xFF000FFu32)?;
                        }
                        canvas.present();
                    } else if keycode == Keycode::B {
                        println!("B down");
                        canvas.circle(50, 50, 30, pixels::Color::RGB(255, 50, 50))?;
                        canvas.present();
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                }

                _ => {}
            }
        }
    }

    Ok(())
}
