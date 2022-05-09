extern crate sdl2;
use crate::common::Point;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;

const KNOB_CENTER: pixels::Color = pixels::Color::RGB(51, 0, 208);
const KNOB: pixels::Color = pixels::Color::RGB(255, 51, 0);
const TEXT: pixels::Color = pixels::Color::RGB(255, 255, 255);

pub struct KnobWidget {
    pub center: Point,
    pub radius: i16,
    pub current_position: i16, //angle - 0° is up
}

impl KnobWidget {
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas
            .filled_pie(
                self.center.x,
                self.center.y,
                self.radius + 10,
                -90,
                self.current_position - 90,
                KNOB,
            )
            .unwrap();

        canvas
            .filled_circle(self.center.x, self.center.y, self.radius, KNOB_CENTER)
            .unwrap();

        canvas
            .circle(self.center.x, self.center.y, self.radius + 11, KNOB_CENTER)
            .unwrap();
        canvas
            .circle(self.center.x, self.center.y, self.radius + 12, KNOB_CENTER)
            .unwrap();

        canvas
            .string(
                self.center.x,
                self.center.y,
                self.current_position.to_string().as_str(),
                TEXT,
            )
            .unwrap();
    }

    pub fn on_click(&mut self) {}

    pub fn is_mouse_inside(&self, x: i32, y: i32) -> bool {
        f32::sqrt(
            f32::powi((self.center.x as i32 - x) as f32, 2)
                + f32::powi((self.center.y as i32 - y) as f32, 2),
        ) <= self.radius as f32
    }

    pub fn change_position(&mut self, move_y: i16) {
        if (move_y >= 0) & (self.current_position < 360) {
            self.current_position += move_y;
        }
        if (move_y <= 0) & (self.current_position > 5) {
            self.current_position += move_y;
        }
    }

    pub fn mouse_is_over(&mut self, canvas: &mut Canvas<Window>) {
        // self.color = pixels::Color::RGB(255, 255, 255);
        self.render(canvas);
    }

    pub fn mouse_is_not_over(&mut self, canvas: &mut Canvas<Window>) {
        // self.color = pixels::Color::RGB(255, 20, 20);
        self.render(canvas);
    }
}
