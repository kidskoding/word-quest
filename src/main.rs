mod ui;
mod title;
mod game;
mod json;
mod lose_screen;
mod round_win_screen;
mod win_screen;
mod instructions;

use macroquad::prelude::*;

#[macroquad::main(conf, "Word Quest")]
async fn main() {
    loop {
        match ui::screen::ScreenManager::current_screen() {
            Some(screen) => {
                match screen {
                    ui::screen::Screen::Title => {
                        title::draw_screen();
                    },
                    ui::screen::Screen::Instructions => {
                        instructions::draw_screen();
                    }
                    ui::screen::Screen::Game => unsafe {
                        game::draw_screen();
                    },
                    ui::screen::Screen::LoseScreen => {
                        lose_screen::draw_screen();
                    }
                    ui::screen::Screen::RoundWinScreen => {
                        round_win_screen::draw_screen();
                    }
                    ui::screen::Screen::WinScreen => {
                        win_screen::draw_screen();
                    }
                }
            }
            None => {
                println!("Error: Failed to get current screen");
                break;
            }
        }

        next_frame().await
    }
}

fn conf() -> Conf {
    Conf {
        window_width: 1280, 
        window_height: 720,
        ..Default::default()
    }
}
