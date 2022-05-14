mod audio_setup;
mod click_widget;
mod common;
mod gui;
mod gui_canvas;
mod knob_widget;
mod kp_sound;
mod metronome_app;
mod metronome_core;
mod nb_widget;
mod ui_widgets;
use std::{sync::mpsc, thread};

use metronome_app::MetronomeApp;
// use std::mem;

use clap::Parser;

// use audio_setup::{host_device_setup, make_strem};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use metronome_core::MetronomeCore;
// use gui_canvas::GUICanvas;

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

fn host_device_setup() -> (cpal::Device, cpal::StreamConfig, cpal::SampleFormat) {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("Default output device is not available");

    println!("Output device : {:?}", device.name());

    let config = device.default_output_config().unwrap();

    println!("Default output config : {:?}", config);

    let format = config.sample_format();

    (device, config.config(), format)
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    //let (device, config, format) = host_device_setup();

    // let (tx, rx) = mpsc::channel();

    let mut metronome_app = MetronomeApp::new_metronome(cli.bpm, cli.time_signature, cli.subdiv);

    println!("done");

    // thread::spawn(move || {
    // let (device, config, format) = host_device_setup();
    // });

    // let sr = config.sample_rate.0 as f32;
    // let nchannels = config.channels as usize;

    // metronome.set_sample_rate(sr);
    // metronome.set_time_per_bar(cli.time_signature);
    // metronome.set_time_subdivision(cli.subdiv);
    // metronome.set_bpm(cli.bpm);
    // metronome.init_score();

    // let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    // let stream = match format {
    //     cpal::SampleFormat::F32 => device.build_output_stream(
    //         &config,
    //         move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
    //             for frame in data.chunks_mut(nchannels) {
    //                 // let s: f32;
    //                 // tx.send(true).unwrap();
    //                 for sample in frame.iter_mut() {
    //                     *sample = 0.0;
    //                 }
    //             }
    //             println!("yey");
    //         },
    //         err_fn,
    //     ),
    //     cpal::SampleFormat::I16 => device.build_output_stream(
    //         &config,
    //         /*move*/
    //         |data: &mut [i16], _: &cpal::OutputCallbackInfo| {},
    //         err_fn,
    //     ),
    //     cpal::SampleFormat::U16 => device.build_output_stream(
    //         &config,
    //         /*move*/
    //         |data: &mut [u16], _: &cpal::OutputCallbackInfo| {},
    //         err_fn,
    //     ),
    // };

    // stream.unwrap().play().unwrap();

    // std::thread::sleep(std::time::Duration::from_millis(3000));

    // metronome.deal_rx();

    // metronome.app_launch();

    // let mut stream = metronome.audio_setup.new_stream(&mut metronome.core);

    // stream.play().unwrap();

    // let mut stream = match make_strem(
    //     &device,
    //     &config,
    //     &format,
    //     cli.bpm,
    //     cli.time_signature,
    //     cli.subdiv,
    // ) {
    //     Ok(stream) => stream,
    //     Err(e) => {
    //         println!("error {}", e);
    //         return Err("Stream not built".to_string());
    //     }
    // };

    // let sdl_context = sdl2::init()?;
    // let video_subsys = sdl_context.video()?;
    // let window = video_subsys
    //     .window(
    //         "rust-sdl2_gfx: draw line & FPSManager",
    //         SCREEN_WIDTH,
    //         SCREEN_HEIGHT,
    //     )
    //     .position_centered()
    //     .opengl()
    //     .build()
    //     .map_err(|e| e.to_string())?;

    // let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
    // canvas.clear();

    // let mut gui_canvas = GUICanvas::new_gui_canvas(SCREEN_HEIGHT as i16, SCREEN_WIDTH as i16, 5, 3);

    // gui_canvas.render_canvas(&mut canvas);
    // canvas.present();

    // let mut events = sdl_context.event_pump()?;

    // 'main: loop {
    //     for event in events.poll_iter() {
    //         match event {
    //             Event::Quit { .. } => break 'main,

    //             Event::KeyDown {
    //                 keycode: Some(keycode),
    //                 ..
    //             } => {
    //                 if keycode == Keycode::Escape {
    //                     break 'main;
    //                 }
    //             }

    //             Event::MouseButtonDown {
    //                 mouse_btn, x, y, ..
    //             } => match mouse_btn {
    //                 MouseButton::Left => {
    //                     if gui_canvas.on_click(x, y) {
    //                         gui_canvas.init_click_widgets(
    //                             gui_canvas
    //                                 .btn_time_per_bar
    //                                 .current_number
    //                                 .try_into()
    //                                 .unwrap(),
    //                             gui_canvas.btn_subdiv.current_number.try_into().unwrap(),
    //                         );
    //                         stream.pause().unwrap();

    //                         //mem::drop(stream);
    //                         stream = metronome.audio_setup.new_stream(&mut metronome.core);

    //                         canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
    //                         canvas.clear();
    //                         gui_canvas.render_canvas(&mut canvas);
    //                         canvas.present();

    //                         //metronome.audio_setup.new_stream(metronome.core);

    //                         // stream = match make_strem(
    //                         //     &device,
    //                         //     &config,
    //                         //     &format,
    //                         //     gui_canvas.knob1.current_position.try_into().unwrap(),
    //                         //     gui_canvas
    //                         //         .btn_time_per_bar
    //                         //         .current_number
    //                         //         .try_into()
    //                         //         .unwrap(),
    //                         //     gui_canvas.btn_subdiv.current_number.try_into().unwrap(),
    //                         // ) {
    //                         //     Ok(stream) => stream,
    //                         //     Err(e) => {
    //                         //         println!("error {}", e);
    //                         //         return Err("Stream not built".to_string());
    //                         //     }
    //                         // };
    //                         stream.play().unwrap();
    //                         // metronome.audio_setup.stream.play().unwrap();
    //                     }
    //                 }
    //                 _ => {}
    //             },

    //             Event::MouseWheel { y, .. } => {
    //                 let mut pos_x: &mut i32 = &mut 0;
    //                 let mut pos_y: &mut i32 = &mut 0;
    //                 unsafe {
    //                     SDL_GetMouseState(pos_x, pos_y);
    //                 }

    //                 if gui_canvas.on_mouse_wheel(*pos_x, *pos_y, y) {
    //                     // metronome.audio_setup.stream.pause().unwrap();
    //                     canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
    //                     canvas.clear();
    //                     gui_canvas.render_canvas(&mut canvas);
    //                     canvas.present();

    //                     metronome.add_bpm(y as usize);

    //                     // stream = match make_strem(
    //                     //     &device,
    //                     //     &config,
    //                     //     &format,
    //                     //     gui_canvas.knob1.current_position.try_into().unwrap(),
    //                     //     gui_canvas
    //                     //         .btn_time_per_bar
    //                     //         .current_number
    //                     //         .try_into()
    //                     //         .unwrap(),
    //                     //     gui_canvas.btn_subdiv.current_number.try_into().unwrap(),
    //                     // ) {
    //                     //     Ok(stream) => stream,
    //                     //     Err(e) => {
    //                     //         println!("error {}", e);
    //                     //         return Err("Stream not built".to_string());
    //                     //     }
    //                     // };

    //                     stream.play().unwrap();
    //                 }
    //             }

    //             Event::MouseMotion { x, y, .. } => {
    //                 // if btn1.is_mouse_inside(x, y) {
    //                 //     btn1.mouse_is_over(&mut canvas);
    //                 // } else {
    //                 //     btn1.mouse_is_not_over(&mut canvas);
    //                 // }
    //                 // if btn2.is_mouse_inside(x, y) {
    //                 //     btn2.mouse_is_over(&mut canvas);
    //                 // } else {
    //                 //     btn2.mouse_is_not_over(&mut canvas);
    //                 // }
    //                 // if btn3.is_mouse_inside(x, y) {
    //                 //     btn3.mouse_is_over(&mut canvas);
    //                 // } else {
    //                 //     btn3.mouse_is_not_over(&mut canvas);
    //                 // }
    //             }

    //             _ => {}
    //         }
    //     }
    // }

    // stream.play().unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(3000));

    Ok(())
}

// fn write_data_f32(data: &mut [f32], metronome: &mut MetronomeApp, nchannels: usize) {
//     for frame in data.chunks_mut(nchannels) {
//         let s: f32;
//         s = metronome.core.get_next_sample();
//         for sample in frame.iter_mut() {
//             *sample = s;
//         }
//     }
// }

// fn write_data_i16(data: &mut [i16], metronome: &mut MetronomeApp, nchannels: usize) {
//     for frame in data.chunks_mut(nchannels) {
//         let s = metronome.core.get_next_sample() as i16;
//         for sample in frame.iter_mut() {
//             *sample = s;
//         }
//     }
// }

// fn write_data_u16(data: &mut [u16], metronome: &mut MetronomeApp, nchannels: usize) {
//     for frame in data.chunks_mut(nchannels) {
//         let s = metronome.core.get_next_sample() as u16;
//         for sample in frame.iter_mut() {
//             *sample = s;
//         }
//     }
// }
