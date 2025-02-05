use macroquad::color::{BLACK, GREEN, RED, WHITE};
use macroquad::input::is_key_pressed;
use macroquad::prelude::{clear_background, draw_text, screen_height, screen_width, KeyCode};
use crate::ui;

pub fn draw_screen() {
    clear_background(GREEN);

    let text = "You Win!";
    let font_size = 80;
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_text(
        text,
        screen_width / 2.0 - (text.len() as f32 * font_size as f32 / 4.0),
        screen_height / 2.0 - 100.0,
        font_size as f32,
        BLACK,
    );

    let start_button = ui::button::Button::new(
        macroquad::prelude::screen_width() / 2.0 - 125.0,
        macroquad::prelude::screen_height() / 2.0 - 50.0,
        200.0,
        50.0,
        WHITE,
        "Play Again".to_string(),
        40.0
    );
    start_button.draw();
    if start_button.is_clicked() || is_key_pressed(KeyCode::Enter) {
        ui::screen::ScreenManager::switch_screen(ui::screen::Screen::Title);
    }

    let start_button = ui::button::Button::new(
        macroquad::prelude::screen_width() / 2.0 - 125.0,
        macroquad::prelude::screen_height() / 2.0 + 50.0,
        200.0,
        50.0,
        WHITE,
        "Quit".to_string(),
        40.0
    );
    start_button.draw();
    if start_button.is_clicked() {
        std::process::exit(0);
    }
}
