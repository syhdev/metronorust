use crate::click_widget::ClickState;
use crate::common::Point;
use crate::knob_widget::KnobWidget;
use sdl2::gfx::primitives::DrawRenderer;

use sdl2::mouse::MouseWheelDirection;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::click_widget::ClickWidget;

const MAIN_CLICK_COLOR: pixels::Color = pixels::Color::RGB(51, 0, 208);
const SECO_CLICK_COLOR: pixels::Color = pixels::Color::RGB(255, 51, 0);

pub struct GUICanvas {
    pub click_widgets: Vec<ClickWidget>,
    pub knob1: KnobWidget,
    center_x: i16,
    center_y: i16,
    metro_radius: i16,
}

impl GUICanvas {
    pub fn new_gui_canvas(
        height: i16,
        width: i16,
        time_per_bar: usize,
        time_subdivision: usize,
    ) -> Self {
        let metro_radius: i16 = height.min(width) - 400;

        let center_x: i16 = width / 2;
        let center_y: i16 = height / 2;

        let nb_clicks: usize = time_per_bar * time_subdivision;

        let angle: f32 = 360.0 / (nb_clicks as f32);

        let mut clicks: Vec<ClickWidget> = vec![];

        let deg_to_rad: f32 = 2.0 * std::f32::consts::PI / 360.0;

        for i in 0..nb_clicks {
            let a = (90.0 - ((i as f32) * angle)) * deg_to_rad;

            let x: f32 = (metro_radius as f32) * a.cos() + center_x as f32;
            let y: f32 = -1.0 * (metro_radius as f32) * a.sin() + center_y as f32;

            let mut color: pixels::Color;
            let radius: i16;

            if i % time_subdivision == 0 {
                color = MAIN_CLICK_COLOR;
                radius = 30;
                if i == 0 {
                    color = pixels::Color::RGB(255, 255, 255);
                }
            } else {
                color = SECO_CLICK_COLOR;
                radius = 25;
            }

            clicks.push(ClickWidget {
                center: Point {
                    x: x as i16,
                    y: y as i16,
                },
                radius: radius,
                color: color,
                state: ClickState::Sound0,
            });
        }

        Self {
            click_widgets: clicks,
            center_x: center_x,
            center_y: center_y,
            metro_radius: metro_radius,
            knob1: {
                KnobWidget {
                    center: Point {
                        x: center_x,
                        y: center_y,
                    },
                    radius: 100,
                    current_position: 120,
                }
            },
        }
    }

    pub fn render_canvas(&mut self, canvas: &mut Canvas<Window>) {
        canvas
            .circle(
                self.center_x,
                self.center_y,
                self.metro_radius,
                pixels::Color::RGB(255, 133, 102),
            )
            .unwrap();

        for i in 0..self.click_widgets.len() {
            self.click_widgets[i].render(canvas);
        }

        self.knob1.render(canvas);
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        // change for to while
        for i in 0..self.click_widgets.len() {
            if self.click_widgets[i].is_mouse_inside(x, y) {
                self.click_widgets[i].on_click();
            }
        }
    }

    pub fn on_mouse_wheel(&mut self, pos_x: i32, pos_y: i32, move_y: i32) {
        if self.knob1.is_mouse_inside(pos_x, pos_y) {
            self.knob1.change_position(move_y as i16);
        }
    }
}
