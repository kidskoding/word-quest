mod ui;
mod title;
mod game;
mod json;

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
                    ui::screen::Screen::Game => {
                        game::draw_screen();
                        game::update();
                    },
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
