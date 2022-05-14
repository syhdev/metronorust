mod audio_setup;
mod click_widget;
mod colors;
mod common;
mod gui_canvas;
mod knob_widget;
mod kp_sound;
mod metronome_core;
mod nb_widget;
mod ui_widgets;
use clap::Parser;

use audio_setup::{host_device_setup, make_strem};
use cpal::traits::StreamTrait;
use gui_canvas::GUICanvas;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels;
use sdl2::sys::SDL_GetMouseState;

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

    canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
    canvas.clear();

    let mut gui_canvas = GUICanvas::new_gui_canvas(SCREEN_HEIGHT as i16, SCREEN_WIDTH as i16, 4, 1);

    let mut stream = match make_strem(
        &device,
        &config,
        &format,
        cli.bpm,
        cli.time_signature,
        cli.subdiv,
        gui_canvas.compute_score(),
    ) {
        Ok(stream) => stream,
        Err(e) => {
            println!("error {}", e);
            return Err("Stream not built".to_string());
        }
    };

    stream.play().unwrap();

    gui_canvas.render_canvas(&mut canvas);
    canvas.present();

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
                    }
                }

                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => match mouse_btn {
                    MouseButton::Left => match gui_canvas.on_click(x, y) {
                        200 => {
                            gui_canvas.init_click_widgets(
                                gui_canvas
                                    .btn_time_per_bar
                                    .current_number
                                    .try_into()
                                    .unwrap(),
                                gui_canvas.btn_subdiv.current_number.try_into().unwrap(),
                            );
                            stream.pause().unwrap();
                            canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
                            canvas.clear();
                            gui_canvas.render_canvas(&mut canvas);
                            canvas.present();

                            stream = match make_strem(
                                &device,
                                &config,
                                &format,
                                gui_canvas.knob1.current_position.try_into().unwrap(),
                                gui_canvas
                                    .btn_time_per_bar
                                    .current_number
                                    .try_into()
                                    .unwrap(),
                                gui_canvas.btn_subdiv.current_number.try_into().unwrap(),
                                gui_canvas.compute_score(),
                            ) {
                                Ok(stream) => stream,
                                Err(e) => {
                                    println!("error {}", e);
                                    return Err("Stream not built".to_string());
                                }
                            };

                            stream.play().unwrap();
                        }
                        999 => {}
                        _ => {
                            stream.pause().unwrap();
                            canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
                            canvas.clear();
                            gui_canvas.render_canvas(&mut canvas);
                            canvas.present();
                            stream = match make_strem(
                                &device,
                                &config,
                                &format,
                                gui_canvas.knob1.current_position.try_into().unwrap(),
                                gui_canvas
                                    .btn_time_per_bar
                                    .current_number
                                    .try_into()
                                    .unwrap(),
                                gui_canvas.btn_subdiv.current_number.try_into().unwrap(),
                                gui_canvas.compute_score(),
                            ) {
                                Ok(stream) => stream,
                                Err(e) => {
                                    println!("error {}", e);
                                    return Err("Stream not built".to_string());
                                }
                            };

                            stream.play().unwrap();
                        }
                    },
                    _ => {}
                },

                Event::MouseWheel { y, .. } => {
                    let mut pos_x: &mut i32 = &mut 0;
                    let mut pos_y: &mut i32 = &mut 0;
                    unsafe {
                        SDL_GetMouseState(pos_x, pos_y);
                    }

                    if gui_canvas.on_mouse_wheel(*pos_x, *pos_y, y) {
                        stream.pause().unwrap();
                        canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
                        canvas.clear();
                        gui_canvas.render_canvas(&mut canvas);
                        canvas.present();

                        stream = match make_strem(
                            &device,
                            &config,
                            &format,
                            gui_canvas.knob1.current_position.try_into().unwrap(),
                            gui_canvas
                                .btn_time_per_bar
                                .current_number
                                .try_into()
                                .unwrap(),
                            gui_canvas.btn_subdiv.current_number.try_into().unwrap(),
                            gui_canvas.compute_score(),
                        ) {
                            Ok(stream) => stream,
                            Err(e) => {
                                println!("error {}", e);
                                return Err("Stream not built".to_string());
                            }
                        };

                        stream.play().unwrap();
                    }
                }

                Event::MouseMotion { x, y, .. } => {
                    // if btn1.is_mouse_inside(x, y) {
                    //     btn1.mouse_is_over(&mut canvas);
                    // } else {
                    //     btn1.mouse_is_not_over(&mut canvas);
                    // }
                    // if btn2.is_mouse_inside(x, y) {
                    //     btn2.mouse_is_over(&mut canvas);
                    // } else {
                    //     btn2.mouse_is_not_over(&mut canvas);
                    // }
                    // if btn3.is_mouse_inside(x, y) {
                    //     btn3.mouse_is_over(&mut canvas);
                    // } else {
                    //     btn3.mouse_is_not_over(&mut canvas);
                    // }
                }

                _ => {}
            }
        }
    }

    // stream.play().unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(3000));

    Ok(())
}
