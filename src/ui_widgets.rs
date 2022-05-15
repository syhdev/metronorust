extern crate sdl2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Point {
    pub x: i16,
    pub y: i16,
}
pub struct ButtonUp {
    pub top_left_corner: Point,
    pub width: i16,
    pub height: i16,
    pub color: pixels::Color,
}

impl ButtonUp {
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas
            .rounded_rectangle(
                self.top_left_corner.x,
                self.top_left_corner.y,
                self.top_left_corner.x + self.width,
                self.top_left_corner.y + self.height,
                10,
                self.color,
            )
            .unwrap();

        canvas.present();
    }

    pub fn on_click(&mut self, canvas: &mut Canvas<Window>) {
        canvas
            .rounded_rectangle(
                self.top_left_corner.x,
                self.top_left_corner.y,
                self.top_left_corner.x + self.width - 5,
                self.top_left_corner.y + self.height + 5,
                10,
                self.color,
            )
            .unwrap();

        canvas.present();
    }

    pub fn is_mouse_inside(&self, x: i32, y: i32) -> bool {
        (x >= self.top_left_corner.x as i32)
            & (x <= (self.top_left_corner.x + self.width) as i32)
            & (y <= (self.top_left_corner.y + self.height) as i32)
            & (y >= self.top_left_corner.y as i32)
    }

    pub fn mouse_is_over(&mut self, canvas: &mut Canvas<Window>) {
        self.color = pixels::Color::RGB(255, 255, 255);
        self.render(canvas);
    }

    pub fn mouse_is_not_over(&mut self, canvas: &mut Canvas<Window>) {
        self.color = pixels::Color::RGB(255, 20, 20);
        self.render(canvas);
    }
}
