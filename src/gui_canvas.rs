use crate::click_widget::ClickState;
use crate::click_widget::ClickWidget;
use crate::common::Point;
use crate::knob_widget::KnobWidget;
use crate::nb_widget::NbWidget;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::render::Canvas;
// use sdl2::video::Window;

use glutin_window::GlutinWindow;
use graphics::*;
use graphics::{Context, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::{AdvancedWindow, Window, WindowSettings};
use touch_visualizer::TouchVisualizer;

use crate::colors::{FIRST_CLICK_COLOR, MAIN_CIRCLE, MAIN_CLICK_COLOR, SECO_CLICK_COLOR};

pub struct GUICanvas {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub rotation: f64,  // Rotation for the square.
    pub knob1: KnobWidget,
    center_x: i16,
    center_y: i16,
    metro_radius: i16,
}

impl GUICanvas {
    pub fn new_gui_canvas(gl: GlGraphics, rotation: f64, width: i16, height: i16) -> Self {
        let metro_radius: i16 = height.min(width) - 400;
        let center_x: i16 = width / 2;
        let center_y: i16 = height / 2;

        let gui_canvas = Self {
            gl,
            center_x: center_x,
            center_y: center_y,
            metro_radius: metro_radius,
            rotation,
            knob1: {
                KnobWidget {
                    center: Point {
                        x: center_x,
                        y: center_y,
                    },
                    radius: 100,
                    current_position: 60,
                }
            },
        };
        gui_canvas
    }

    pub fn render(&mut self, args: &RenderArgs) {
        // use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // let square = rectangle::square(0.0, 0.0, 50.0);
        // let square2 = rectangle::square(0.0, 50.0, 50.0);
        // let rotation = self.rotation;
        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            self.knob1.render(c, gl);

            // let transform = c
            //     .transform
            //     .trans(x, y)
            //     .rot_rad(rotation)
            //     .trans(-25.0, -25.0);

            // // Draw a box rotating around the middle of the screen.
            // rectangle(RED, square, transform, gl);
            // ellipse(RED, square2, c.transform.trans(x, y), gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

// pub struct GUICanvas {
//     pub click_widgets: Vec<ClickWidget>,
//     pub knob1: KnobWidget,
//     pub btn_time_per_bar: NbWidget,
//     pub btn_subdiv: NbWidget,
//     center_x: i16,
//     center_y: i16,
//     metro_radius: i16,
// }

// impl GUICanvas {
//     pub fn new_gui_canvas(
//         height: i16,
//         width: i16,
//         time_per_bar: usize,
//         time_subdivision: usize,
//     ) -> Self {
//         let metro_radius: i16 = height.min(width) - 400;
//         let center_x: i16 = width / 2;
//         let center_y: i16 = height / 2;

//         let mut gui_canvas = Self {
//             click_widgets: vec![],
//             center_x: center_x,
//             center_y: center_y,
//             metro_radius: metro_radius,
//             knob1: {
//                 KnobWidget {
//                     center: Point {
//                         x: center_x,
//                         y: center_y,
//                     },
//                     radius: 100,
//                     current_position: 60,
//                 }
//             },
//             btn_time_per_bar: {
//                 NbWidget {
//                     top_left_corner: Point { x: 50, y: 50 },
//                     width: 150,
//                     height: 110,
//                     current_number: 4,
//                 }
//             },
//             btn_subdiv: {
//                 NbWidget {
//                     top_left_corner: Point { x: 600, y: 50 },
//                     width: 150,
//                     height: 110,
//                     current_number: 1,
//                 }
//             },
//         };

//         Self::init_click_widgets(&mut gui_canvas, time_per_bar, time_subdivision);

//         gui_canvas
//     }

//     pub fn init_click_widgets(&mut self, time_per_bar: usize, time_subdivision: usize) {
//         let nb_clicks: usize = time_per_bar * time_subdivision;

//         let angle: f32 = 360.0 / (nb_clicks as f32);

//         let mut clicks: Vec<ClickWidget> = vec![];

//         let deg_to_rad: f32 = 2.0 * std::f32::consts::PI / 360.0;

//         for i in 0..nb_clicks {
//             let a = (90.0 - ((i as f32) * angle)) * deg_to_rad;

//             let x: f32 = (self.metro_radius as f32) * a.cos() + self.center_x as f32;
//             let y: f32 = -1.0 * (self.metro_radius as f32) * a.sin() + self.center_y as f32;

//             let mut color: pixels::Color;
//             let radius: i16;
//             let mut state: ClickState;

//             if i % time_subdivision == 0 {
//                 color = MAIN_CLICK_COLOR;
//                 radius = 30;
//                 state = ClickState::Sound2;
//                 if i == 0 {
//                     color = FIRST_CLICK_COLOR;
//                     state = ClickState::Sound3;
//                 }
//             } else {
//                 color = SECO_CLICK_COLOR;
//                 radius = 20;
//                 state = ClickState::Sound1
//             }

//             clicks.push(ClickWidget {
//                 center: Point {
//                     x: x as i16,
//                     y: y as i16,
//                 },
//                 radius: radius,
//                 color: color,
//                 state: state,
//             });
//         }

//         self.click_widgets = clicks;
//     }

//     pub fn compute_score(&mut self) -> Vec<usize> {
//         let mut score: Vec<usize> = vec![];
//         for i in 0..self.click_widgets.len() {
//             match self.click_widgets[i].state {
//                 ClickState::Sound0 => score.push(0),
//                 ClickState::Sound1 => score.push(3),
//                 ClickState::Sound2 => score.push(2),
//                 ClickState::Sound3 => score.push(1),
//             }
//         }
//         score
//     }

//     pub fn render_canvas(&mut self, canvas: &mut Canvas<Window>) {
//         canvas
//             .circle(self.center_x, self.center_y, self.metro_radius, MAIN_CIRCLE)
//             .unwrap();

//         for i in 0..self.click_widgets.len() {
//             self.click_widgets[i].render(canvas);
//         }

//         self.knob1.render(canvas);

//         self.btn_time_per_bar.render(canvas);

//         self.btn_subdiv.render(canvas);
//     }

//     pub fn on_click(&mut self, x: i32, y: i32) -> usize {
//         for i in 0..self.click_widgets.len() {
//             if self.click_widgets[i].is_mouse_inside(x, y) {
//                 self.click_widgets[i].on_click();
//                 return i;
//             }
//         }
//         if self.btn_time_per_bar.is_mouse_inside(x, y) {
//             self.btn_time_per_bar.on_click(x as i16, y as i16);
//             return 200;
//         } else if self.btn_subdiv.is_mouse_inside(x, y) {
//             self.btn_subdiv.on_click(x as i16, y as i16);
//             return 200;
//         }

//         return 999;
//     }

//     pub fn on_mouse_wheel(&mut self, pos_x: i32, pos_y: i32, move_y: i32) -> bool {
//         if self.knob1.is_mouse_inside(pos_x, pos_y) {
//             self.knob1.change_position(move_y as i16);
//             true
//         } else {
//             false
//         }
//     }
// }
