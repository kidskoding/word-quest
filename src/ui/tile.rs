use macroquad::prelude::*;

use crate::ui;

pub struct Tile {
    letter: char,
    button: ui::button::Button
}

impl Tile {
    pub fn new(letter: char, x: f32, y: f32) -> Tile {
        let button = ui::button::Button::new(x, y, 
                                             100.0, 100.0,
                                             WHITE, letter.to_string(), 40.0);
        Tile {
            letter,
            button,
        }
    }
    
    pub fn get_button(&mut self) -> &mut ui::button::Button {
        &mut self.button
    }
    pub fn draw(&self) { self.button.draw(); }
    pub fn is_clicked(&self) -> bool { self.button.is_clicked() }
    pub fn get_letter(&self) -> char {
        self.letter
    }
}
