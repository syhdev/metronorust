extern crate sdl2;
use crate::common::Point;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum ClickState {
    Sound0, //no sound
    Sound1,
    Sound2,
    Sound3,
}

pub struct ClickWidget {
    pub center: Point,
    pub radius: i16,
    pub color: pixels::Color,
    pub state: ClickState,
}

impl ClickWidget {
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        match self.state {
            ClickState::Sound0 => {
                canvas
                    .circle(self.center.x, self.center.y, self.radius - 15, self.color)
                    .unwrap();
            }
            ClickState::Sound1 => {
                canvas
                    .circle(self.center.x, self.center.y, self.radius - 10, self.color)
                    .unwrap();
            }
            ClickState::Sound2 => {
                canvas
                    .circle(self.center.x, self.center.y, self.radius - 5, self.color)
                    .unwrap();
            }
            ClickState::Sound3 => {
                canvas
                    .circle(self.center.x, self.center.y, self.radius, self.color)
                    .unwrap();
            }
        }
    }

    pub fn on_click(&mut self) {
        match self.state {
            ClickState::Sound0 => {
                println!("now sound1");
                self.state = ClickState::Sound1;
            }
            ClickState::Sound1 => {
                println!("now sound2");
                self.state = ClickState::Sound2;
            }
            ClickState::Sound2 => {
                println!("now sound3");
                self.state = ClickState::Sound3;
            }
            ClickState::Sound3 => {
                println!("now sound0");
                self.state = ClickState::Sound0;
            }
        }
    }

    pub fn is_mouse_inside(&self, x: i32, y: i32) -> bool {
        f32::sqrt(
            f32::powi((self.center.x as i32 - x) as f32, 2)
                + f32::powi((self.center.y as i32 - y) as f32, 2),
        ) <= self.radius as f32
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
