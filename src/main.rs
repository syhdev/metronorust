mod click_widget;
mod colors;
mod gui_canvas;
mod knob_widget;
mod kp_sound;
mod metronome_app;
mod metronome_core;
mod misc;
mod nb_widget;
mod state;
mod ui_widgets;
use clap::Parser;
use rand::Rng;

use ring_channel::*;
use std::num::NonZeroUsize;

// use crate::colors::BACKGROUND_COLOR;

// use gui_canvas::GUICanvas;

// extern crate sdl2;
use metronome_app::MetronomeApp;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::mouse::MouseButton;
// use sdl2::sys::SDL_GetMouseState;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::misc::*;

use crate::state::*;

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

async fn run() {
    let cli = Cli::parse();

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    let (mut sender, mut receiver) = ring_channel(NonZeroUsize::new(1).unwrap());

    let metronome_app: MetronomeApp = MetronomeApp::create_metronome_app(
        cli.bpm,
        cli.time_signature,
        cli.subdiv,
        vec![1; 4],
        sender,
    );

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let mut rng = rand::thread_rng();
            let mut vec: Vec<f32> = Vec::with_capacity(1024);
            for _ in 0..vec.capacity() {
                vec.push(rng.gen_range(0.0..1.0));
            }
            // let a: [f32; 1024] = vec.as_slice().try_into().unwrap();
            let a: [f32; 1024] = receiver.recv().unwrap();
            state.update(a);
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}

fn main() -> Result<(), String> {
    // let cli = Cli::parse();

    pollster::block_on(run());

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

    // canvas.set_draw_color(BACKGROUND_COLOR);
    // canvas.clear();

    // let mut gui_canvas = GUICanvas::new_gui_canvas(SCREEN_HEIGHT as i16, SCREEN_WIDTH as i16, 4, 1);

    // let metronome_app: MetronomeApp = MetronomeApp::create_metronome_app(
    //     cli.bpm,
    //     cli.time_signature,
    //     cli.subdiv,
    //     gui_canvas.compute_score(),
    // );

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
    //                 MouseButton::Left => match gui_canvas.on_click(x, y) {
    //                     200 => {
    //                         let time_per_bar = gui_canvas
    //                             .btn_time_per_bar
    //                             .current_number
    //                             .try_into()
    //                             .unwrap();

    //                         let subdiv = gui_canvas.btn_subdiv.current_number.try_into().unwrap();

    //                         let bpm = gui_canvas.knob1.current_position.try_into().unwrap();

    //                         gui_canvas.init_click_widgets(time_per_bar, subdiv);
    //                         canvas.set_draw_color(BACKGROUND_COLOR);
    //                         canvas.clear();
    //                         gui_canvas.render_canvas(&mut canvas);
    //                         canvas.present();

    //                         let score = gui_canvas.compute_score();

    //                         metronome_app.metronome_core.lock().unwrap().setup(
    //                             time_per_bar,
    //                             subdiv,
    //                             bpm,
    //                             score,
    //                         );
    //                     }
    //                     999 => {}
    //                     _ => {
    //                         canvas.set_draw_color(BACKGROUND_COLOR);
    //                         canvas.clear();
    //                         gui_canvas.render_canvas(&mut canvas);
    //                         canvas.present();

    //                         let time_per_bar = gui_canvas
    //                             .btn_time_per_bar
    //                             .current_number
    //                             .try_into()
    //                             .unwrap();

    //                         let subdiv = gui_canvas.btn_subdiv.current_number.try_into().unwrap();

    //                         let bpm = gui_canvas.knob1.current_position.try_into().unwrap();

    //                         canvas.set_draw_color(BACKGROUND_COLOR);
    //                         canvas.clear();
    //                         gui_canvas.render_canvas(&mut canvas);
    //                         canvas.present();

    //                         let score = gui_canvas.compute_score();

    //                         metronome_app.metronome_core.lock().unwrap().setup(
    //                             time_per_bar,
    //                             subdiv,
    //                             bpm,
    //                             score,
    //                         );
    //                     }
    //                 },
    //                 _ => {}
    //             },

    //             Event::MouseWheel { y, .. } => {
    //                 let pos_x: &mut i32 = &mut 0;
    //                 let pos_y: &mut i32 = &mut 0;
    //                 unsafe {
    //                     SDL_GetMouseState(pos_x, pos_y);
    //                 }

    //                 if gui_canvas.on_mouse_wheel(*pos_x, *pos_y, y) {
    //                     canvas.set_draw_color(BACKGROUND_COLOR);
    //                     canvas.clear();
    //                     gui_canvas.render_canvas(&mut canvas);
    //                     canvas.present();

    //                     let time_per_bar = gui_canvas
    //                         .btn_time_per_bar
    //                         .current_number
    //                         .try_into()
    //                         .unwrap();

    //                     let subdiv = gui_canvas.btn_subdiv.current_number.try_into().unwrap();

    //                     let bpm = gui_canvas.knob1.current_position.try_into().unwrap();

    //                     gui_canvas.init_click_widgets(time_per_bar, subdiv);
    //                     canvas.set_draw_color(BACKGROUND_COLOR);
    //                     canvas.clear();
    //                     gui_canvas.render_canvas(&mut canvas);
    //                     canvas.present();

    //                     let score = gui_canvas.compute_score();

    //                     metronome_app.metronome_core.lock().unwrap().setup(
    //                         time_per_bar,
    //                         subdiv,
    //                         bpm,
    //                         score,
    //                     );
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

    Ok(())
}
