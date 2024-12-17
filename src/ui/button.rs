use macroquad::prelude::*;

pub struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    text: String,
    font_size: f32,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, 
               color: Color, text: String, font_size: f32) -> Button {
        Button {
            x,
            y,
            width,
            height,
            color,
            text,
            font_size,
        }
    }
    
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
        let text_dim = measure_text(&self.text, None,
                                    self.font_size as u16, 1.0);
        draw_text(
            &self.text,
            self.x + self.width / 2.0 - text_dim.width / 2.0,
            self.y + self.height / 2.0 - text_dim.height / 2.0 + 20.0,
            self.font_size,
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
    
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_width(&self) -> f32 {
        self.width
    }
    pub fn get_height(&self) -> f32 {
        self.height
    }
    
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}
