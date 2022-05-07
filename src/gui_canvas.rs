use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::click_widget::ClickWidget;

pub struct GUICanvas {
    pub click_widgets: Vec<ClickWidget>,
}

impl GUICanvas {
    pub fn render_canvas(&mut self, canvas: &mut Canvas<Window>) {
        for i in 0..self.click_widgets.len() {
            self.click_widgets[i].render(canvas);
        }
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        // change for to while
        for i in 0..self.click_widgets.len() {
            if self.click_widgets[i].is_mouse_inside(x, y) {
                self.click_widgets[i].on_click();
            }
        }
    }
}
