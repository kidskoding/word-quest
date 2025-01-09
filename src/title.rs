use macroquad::prelude::*;

use crate::ui;

pub fn draw_screen() {
    clear_background(DARKGRAY);

    let title = "Word Quest";
    let title_dim = measure_text(title, None, 80, 1.0);
    draw_text(
        title,
        screen_width() / 2.0 - title_dim.width / 2.0,
        screen_height() / 2.0 - title_dim.height / 2.0 - 100.0,
        80.0,
        WHITE
    );

    let author = "by kidskoding";
    let author_dim = measure_text(author, None, 30, 1.0);
    draw_text(
        author,
        screen_width() / 2.0 - author_dim.width / 2.0,
        screen_height() / 2.0 - author_dim.height / 2.0 - 70.0,
        30.0,
        WHITE
    );

    let start_button = ui::button::Button::new(
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0,
        200.0,
        50.0,
        GOLD,
        "Play".to_string(),
        40.0
    );
    start_button.draw();
    if start_button.is_clicked() || is_key_pressed(KeyCode::Enter) {
        ui::screen::ScreenManager::switch_screen(ui::screen::Screen::Instructions);
    }

    let quit_button = ui::button::Button::new(
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 + 75.0,
        200.0,
        50.0,
        GOLD,
        "Quit".to_string(),
        40.0
    );
    quit_button.draw();
    if quit_button.is_clicked() {
        std::process::exit(0);
    }
}
