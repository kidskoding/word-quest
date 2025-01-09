use macroquad::prelude::*;
use crate::ui::button::Button;
use crate::ui::screen::{Screen, ScreenManager};

pub fn draw_screen() {
    clear_background(DARKGRAY);

    let box_width = 800.0;
    let box_x = screen_width() / 2.0 - box_width / 2.0;
    draw_rectangle(box_x, 50.0, box_width, 590.0, WHITE);

    let text_color = BLACK;
    let header_size = 30.0;
    let text_size = 25.0;
    let mut y_pos = 80.0;
    let padding = 40.0;

    draw_text("Instructions", box_x + 20.0, y_pos, header_size, ORANGE);
    y_pos += padding;
    draw_text(
        "Create words using the given letter tiles to reach the target score.",
        box_x + 40.0, y_pos, text_size, text_color
    );
    y_pos += padding - 10.0;
    draw_text(
        "You have 4 words per round to meet the score requirement",
        box_x + 40.0, y_pos, text_size, text_color
    );

    y_pos += padding + 10.0;
    draw_text("Scoring System", box_x + 20.0, y_pos, header_size, BLUE);
    y_pos += padding;
    let score_texts = [
        "• Common Letters (10 pts): E, A, T",
        "• Regular Letters (15 pts): H, I, N, O, S, R",
        "• Medium Letters (20 pts): D, L",
        "• Less Common (25 pts): U, F, M, C, G, Y",
        "• Rare Letters (30 pts): P, B, W",
        "• Very Rare (35 pts): V, K, J, X, Q, Z",
        "• Word Score = Total Score of Unique Characters * Word Length"
    ];

    for text in score_texts.iter() {
        draw_text(text, box_x + 40.0, y_pos, text_size, text_color);
        y_pos += padding - 10.0;
    }

    y_pos += 10.0;
    draw_text("Controls", box_x + 20.0, y_pos, header_size, GREEN);
    y_pos += padding;
    let control_texts = [
        "• Click tiles or type letters to build words",
        "• Press ENTER or click 'Play Word' to submit",
        "• Press BACKSPACE or 'X' to clear current word",
        "• Use 'Shuffle' to rearrange tiles",
        "• Use 'Discard' to get new tiles (3 per round)"
    ];

    for text in control_texts.iter() {
        draw_text(text, box_x + 40.0, y_pos, text_size, text_color);
        y_pos += padding - 10.0;
    }

    let play_button = Button::new(
        screen_width() / 2.0 - 100.0,
        650.0,
        200.0,
        50.0,
        BLUE,
        "Play".to_string(),
        30.0
    );
    play_button.draw();

    if play_button.is_clicked() || is_key_pressed(KeyCode::Enter) {
        ScreenManager::switch_screen(Screen::Game);
    }
}
