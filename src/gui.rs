extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::Canvas;
use sdl2::sys::SDL_GetMouseState;
use sdl2::video::Window;
use sdl2::{pixels, video, Sdl, VideoSubsystem};

use crate::gui_canvas::GUICanvas;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct GUI {
    sdl_context: Sdl,
    video_subsys: VideoSubsystem,
    // window: Window,
    canvas: Canvas<Window>,
    gui_canvas: GUICanvas,
}

impl GUI {
    pub fn create_gui() -> GUI {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "rust-sdl2_gfx: draw line & FPSManager",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
        canvas.clear();

        let mut gui_canvas =
            GUICanvas::new_gui_canvas(SCREEN_HEIGHT as i16, SCREEN_WIDTH as i16, 5, 3);

        gui_canvas.render_canvas(&mut canvas);
        canvas.present();

        GUI {
            sdl_context: sdl_context,
            video_subsys: video_subsys,
            // window: window,
            canvas: canvas,
            gui_canvas: gui_canvas,
        }
    }

    pub fn app_main_loop(&mut self) {
        let mut events = self.sdl_context.event_pump().unwrap();

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
                        MouseButton::Left => {
                            if self.gui_canvas.on_click(x, y) {
                                self.gui_canvas.init_click_widgets(
                                    self.gui_canvas
                                        .btn_time_per_bar
                                        .current_number
                                        .try_into()
                                        .unwrap(),
                                    self.gui_canvas
                                        .btn_subdiv
                                        .current_number
                                        .try_into()
                                        .unwrap(),
                                );
                                //stream.pause().unwrap();

                                //mem::drop(stream);
                                //stream = metronome.audio_setup.new_stream(&mut metronome.core);

                                self.canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
                                self.canvas.clear();
                                self.gui_canvas.render_canvas(&mut self.canvas);
                                self.canvas.present();

                                //stream.play().unwrap();
                                // metronome.audio_setup.stream.play().unwrap();
                            }
                        }
                        _ => {}
                    },

                    Event::MouseWheel { y, .. } => {
                        let mut pos_x: &mut i32 = &mut 0;
                        let mut pos_y: &mut i32 = &mut 0;
                        unsafe {
                            SDL_GetMouseState(pos_x, pos_y);
                        }

                        if self.gui_canvas.on_mouse_wheel(*pos_x, *pos_y, y) {
                            // metronome.audio_setup.stream.pause().unwrap();
                            self.canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
                            self.canvas.clear();
                            self.gui_canvas.render_canvas(&mut self.canvas);
                            self.canvas.present();

                            //metronome.add_bpm(y as usize);

                            //stream.play().unwrap();
                        }
                    }

                    Event::MouseMotion { x, y, .. } => {}

                    _ => {}
                }
            }
        }
    }
}
