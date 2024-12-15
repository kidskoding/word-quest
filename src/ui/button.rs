use macroquad::prelude::*;
pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    text: String,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, text: String) -> Button {
        Button {
            x,
            y,
            width,
            height,
            color,
            text,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
        let text_dim = measure_text(&self.text, None, 40, 1.0);
        draw_text(
            &self.text,
            self.x + self.width / 2.0 - text_dim.width / 2.0,
            self.y + self.height / 2.0 - text_dim.height / 2.0 + 20.0,
            40.0,
            BLACK
        );
    }
    
    pub fn is_clicked(&self) -> bool {
        let mouse_pos = mouse_position();
        
        is_mouse_button_pressed(MouseButton::Left)
            && mouse_pos.0 >= self.x 
            && mouse_pos.0 <= self.x + self.width 
            && mouse_pos.1 >= self.y 
            && mouse_pos.1 <= self.y + self.height
    }
}
