extern crate sdl2;
use crate::colors::{BTN, TEXT};
use crate::common::Point;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct NbWidget {
    pub top_left_corner: Point,
    pub width: i16,
    pub height: i16,
    pub current_number: i16,
}

impl NbWidget {
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas
            .rounded_rectangle(
                self.top_left_corner.x,
                self.top_left_corner.y,
                self.top_left_corner.x + self.width,
                self.top_left_corner.y + self.height,
                10,
                BTN,
            )
            .unwrap();

        canvas
            .rounded_box(
                self.top_left_corner.x + 2 * self.width / 3,
                self.top_left_corner.y + 10,
                self.top_left_corner.x + 2 * self.width / 3 + 40,
                self.top_left_corner.y + 50,
                5,
                BTN,
            )
            .unwrap();

        canvas
            .rounded_box(
                self.top_left_corner.x + 2 * self.width / 3,
                self.top_left_corner.y + 60,
                self.top_left_corner.x + 2 * self.width / 3 + 40,
                self.top_left_corner.y + 100,
                5,
                BTN,
            )
            .unwrap();

        canvas
            .string(
                self.top_left_corner.x + self.width / 3,
                self.top_left_corner.y + self.height / 2,
                self.current_number.to_string().as_str(),
                TEXT,
            )
            .unwrap();
    }

    pub fn on_click(&mut self, x: i16, y: i16) {
        if (x >= self.top_left_corner.x + 2 * self.width / 3)
            & (x <= self.top_left_corner.x + 2 * self.width / 3 + 40)
            & (y >= self.top_left_corner.y + 10)
            & (y <= self.top_left_corner.y + 50)
        {
            self.current_number += 1;
        } else if (x >= self.top_left_corner.x + 2 * self.width / 3)
            & (x <= self.top_left_corner.x + 2 * self.width / 3 + 40)
            & (y >= self.top_left_corner.y + 60)
            & (y <= self.top_left_corner.y + 100)
        {
            if self.current_number >= 2 {
                self.current_number -= 1;
                println!("{}", self.current_number);
            }
        }
    }

    pub fn is_mouse_inside(&self, x: i32, y: i32) -> bool {
        (x as i16 >= self.top_left_corner.x)
            & (x as i16 <= self.top_left_corner.x + self.width)
            & (y as i16 >= self.top_left_corner.y)
            & (y as i16 <= self.top_left_corner.y + self.height)
    }

    pub fn change_position(&mut self, move_y: i16) {
        // if (move_y >= 0) & (self.current_position < 360) {
        //     self.current_position += move_y;
        // }
        // if (move_y <= 0) & (self.current_position > 5) {
        //     self.current_position += move_y;
        // }
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
