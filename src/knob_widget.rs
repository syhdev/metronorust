// extern crate sdl2;
use crate::colors::{KNOB, KNOB_CENTER, TEXT};
use crate::common::Point;
// use sdl2::gfx::primitives::DrawRenderer;
// use sdl2::render::Canvas;
// use sdl2::video::Window;

use sfml::graphics::VertexArray;
use sfml::{
    graphics::{
        CircleShape, Color, Font, PrimitiveType, Rect, RenderStates, RenderTarget, RenderWindow,
        Shape, Text, Texture, Transform, Transformable, Vertex, View,
    },
    system::{Clock, Vector2, Vector2f, Vector2i},
    window::{mouse::Button, ContextSettings, Event, Key, Style, VideoMode},
};

pub struct KnobWidget {
    pub center: Point,
    pub radius: i16,
    pub current_position: i16, //angle - 0° is up
}

impl KnobWidget {
    pub fn render(&mut self, window: &mut RenderWindow) {
        // canvas
        //     .filled_pie(
        //         self.center.x,
        //         self.center.y,
        //         self.radius + 20,
        //         -90,
        //         self.current_position - 90,
        //         KNOB,
        //     )
        //     .unwrap();

        let mut ball = CircleShape::default();
        ball.set_radius(self.radius as f32);
        ball.set_outline_thickness(3.);
        ball.set_outline_color(Color::BLACK);
        ball.set_fill_color(Color::WHITE);
        ball.set_origin((self.radius as f32, self.radius as f32));
        ball.set_position((self.center.x as f32, self.center.y as f32));

        let n = 30;
        let mut vertices: Vec<Vertex> = vec![];
        vertices.push(Vertex::new(
            Vector2f::new(self.center.x as f32, self.center.y as f32),
            Color::RED,
            Vector2f::new(0., 0.),
        ));
        for i in 1..=(n + 1) {
            vertices.push(Vertex::new(
                Vector2f::new(
                    self.center.x as f32
                        + (self.radius as f32)
                            * f32::cos(2.0 * std::f32::consts::PI / (n as f32) * i as f32),
                    self.center.y as f32
                        + (self.radius as f32)
                            * f32::sin(2.0 * std::f32::consts::PI / (n as f32) * i as f32),
                ),
                Color::RED,
                Vector2f::new(0., 0.),
            ));
        }

        // canvas
        //     .filled_circle(self.center.x, self.center.y, self.radius, KNOB_CENTER)
        //     .unwrap();

        // canvas
        //     .circle(self.center.x, self.center.y, self.radius + 21, KNOB_CENTER)
        //     .unwrap();
        // canvas
        //     .circle(self.center.x, self.center.y, self.radius + 22, KNOB_CENTER)
        //     .unwrap();

        // canvas
        //     .string(
        //         self.center.x,
        //         self.center.y,
        //         self.current_position.to_string().as_str(),
        //         TEXT,
        //     )
        //     .unwrap();
        // window.draw(&ball);
        window.draw_primitives(
            &vertices,
            PrimitiveType::TRIANGLE_FAN,
            &RenderStates::DEFAULT,
        );
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

    // pub fn mouse_is_over(&mut self, canvas: &mut Canvas<Window>) {
    //     // self.color = pixels::Color::RGB(255, 255, 255);
    //     self.render(canvas);
    // }

    // pub fn mouse_is_not_over(&mut self, canvas: &mut Canvas<Window>) {
    //     // self.color = pixels::Color::RGB(255, 20, 20);
    //     self.render(canvas);
    // }
}
