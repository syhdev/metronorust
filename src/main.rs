mod click_widget;
mod colors;
mod common;
mod gui_canvas;
mod knob_widget;
mod kp_sound;
mod metronome_app;
mod metronome_core;
mod nb_widget;
mod ui_widgets;
use clap::Parser;

use crate::colors::BACKGROUND_COLOR;

use gui_canvas::GUICanvas;

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
// extern crate sdl2_window;
extern crate touch_visualizer;
use glutin_window::GlutinWindow;
use graphics::*;
use graphics::{Context, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::{AdvancedWindow, Window, WindowSettings};
use touch_visualizer::TouchVisualizer;

extern crate sdl2;
use metronome_app::MetronomeApp;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::sys::SDL_GetMouseState;
// use sdl2_window::Sdl2Window;

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

    let mut window: GlutinWindow =
        WindowSettings::new("piston-example-user_input", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .exit_on_esc(true)
            .graphics_api(OpenGL::V4_5)
            .build()
            .unwrap();

    let opengl = OpenGL::V3_2;
    // Create a new game and run it.
    let mut app = GUICanvas::new_gui_canvas(
        GlGraphics::new(opengl),
        0.0,
        SCREEN_WIDTH as i16,
        SCREEN_HEIGHT as i16,
    );

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }

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
