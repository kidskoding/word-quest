use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use lazy_static::lazy_static;
use ::rand::Rng;
use crate::ui;

lazy_static! {
    static ref scoring: HashMap<char, i32> = {
        let mut map = HashMap::new();
        map.insert('e', 10); map.insert('a', 10); map.insert('t', 10);

        map.insert('h', 15); map.insert('i', 15); map.insert('n', 15);
        map.insert('o', 15); map.insert('s', 15); map.insert('r', 15);

        map.insert('d', 20); map.insert('l', 20);

        map.insert('u', 25); map.insert('f', 25); map.insert('m', 25);
        map.insert('c', 25); map.insert('g', 25); map.insert('y', 25);

        map.insert('p', 30); map.insert('b', 30); map.insert('w', 30);

        map.insert('v', 35); map.insert('k', 35); map.insert('j', 35);
        map.insert('x', 35); map.insert('q', 35); map.insert('z', 35);

        map
    };
    static ref words: Vec<char> = scoring.keys().cloned().collect();
    static ref tiles: Mutex<Vec<ui::tile::Tile>> = Mutex::new(Vec::new());
    static ref current_word: Mutex<String> = Mutex::new(String::new());
}

pub fn draw_screen() {
    clear_background(DARKGRAY);
    
    draw_rectangle(
        100.0,
        100.0,
        450.0,
        475.0,
        WHITE
    );
    
    draw_tiles();

    draw_rectangle(
        screen_width() / 2.0 + 50.0,
        screen_height() - 225.0,
        475.0,
        50.0,
        WHITE
    );

    if let Ok(word) = current_word.lock() {
        let letter_dim = measure_text(&word, None, 40, 1.0);
        draw_text(
            &word,
            screen_width() / 2.0 + 50.0 + 475.0 / 2.0 - letter_dim.width / 2.0,
            screen_height() - 225.0 + 35.0,
            40.0,
            BLACK
        );
    }
    
    let play_button = ui::button::Button::new(
        screen_width() / 2.0 + 200.0,
        screen_height() - 150.0,
        475.0 / 3.0,
        50.0,
        GREEN,
        "Play Word".to_string(),
        35.0
    );
    play_button.draw();
    if play_button.is_clicked() || is_key_pressed(KeyCode::Enter) {
        
    }
    
    let shuffle_button = ui::button::Button::new(
        screen_width() / 2.0 + 50.0 + 60.0,
        screen_height() - 75.0,
        475.0 / 3.0,
        50.0,
        YELLOW,
        "Shuffle".to_string(),
        40.0
    );
    shuffle_button.draw();
    if shuffle_button.is_clicked() {
        shuffle_tiles();
    }
    
    let discard_button = ui::button::Button::new(
        screen_width() / 2.0 + 50.0 + 235.0,
        screen_height() - 75.0,
        475.0 / 3.0,
        50.0,
        RED,
        "Discard".to_string(),
        40.0
    );
    discard_button.draw();
    if discard_button.is_clicked() {
        discard_tiles();
    }
}

fn draw_tiles() {
    if let Ok(mut guard) = tiles.lock() {
        if guard.is_empty() {
            let mut start_y = 100.0;
            let mut rand_indexes: Vec<usize> = Vec::new();
            let mut rng = ::rand::thread_rng();
            
            while rand_indexes.len() < 12 {
                let rand_index = rng.gen_range(0..words.len());
                if !rand_indexes.contains(&rand_index) {
                    rand_indexes.push(rand_index);
                }
            }
            
            let mut inc = 0;
            for r in 0..3 {
                let mut start_x = screen_width() / 2.0 + 50.0;
                for c in 0..4 {
                    let tile = ui::tile::Tile::new(*words.get(rand_indexes[inc * 4 + c]).unwrap(), 
                                                   start_x, start_y);
                    guard.push(tile);
                    start_x += 125.0;
                }
                start_y += 125.0;
                inc += 1;
            }
        }
    }
    
    if let Ok(guard) = tiles.lock() {
        for tile in guard.iter() {
            tile.draw();
        }
    } else {
        println!("Error: Failed to get tiles lock");
    }
}

fn redraw(guard: &mut Vec<ui::tile::Tile>) {
    let mut start_y = 100.0;
    let mut inc = 0;
    for r in 0..3 {
        let mut start_x = screen_width() / 2.0 + 50.0;
        for c in 0..4 {
            let tile = &mut guard[inc * 4 + c];
            tile.get_button().set_x(start_x);
            tile.get_button().set_y(start_y);
            start_x += 125.0;
        }
        start_y += 125.0;
        inc += 1;
    }
}

fn shuffle_tiles() {
    if let Ok(mut guard) = tiles.lock() {
        for i in 0..guard.len() {
            let j = rand::gen_range(0, guard.len());
            guard.swap(i, j);
        }
        redraw(&mut guard);
    } else {
        println!("Error: Failed to get tiles lock");
    }
}

fn discard_tiles() {
    if let Ok(mut guard) = tiles.lock() {
        guard.clear();
    } else {
        println!("Error: Failed to get tiles lock");
    }
}

fn clear_word() {
    if let Ok(mut word) = current_word.lock() {
        word.clear();
    } else {
        println!("Error: Failed to get current word lock");
    }
}

pub fn update() {
    if let Ok(mut word) = current_word.lock() {
        if let Ok(mut guard) = tiles.lock() {
            for tile in guard.iter_mut() {
                let tile_letter = tile.get_letter();
                if let Some(key_code) = char_to_key_code(tile_letter) {
                    if is_key_pressed(key_code) || tile.is_clicked() {
                        word.push(tile_letter);
                    }
                }
            }
        } else {
            println!("Error: Failed to get tiles lock");
        }
    } else {
        println!("Error: Failed to get current word lock");
    }
}
fn char_to_key_code(c: char) -> Option<KeyCode> {
    match c {
        'a' => Some(KeyCode::A), 'b' => Some(KeyCode::B), 'c' => Some(KeyCode::C),
        'd' => Some(KeyCode::D), 'e' => Some(KeyCode::E), 'f' => Some(KeyCode::F),
        'g' => Some(KeyCode::G), 'h' => Some(KeyCode::H), 'i' => Some(KeyCode::I),
        'j' => Some(KeyCode::J), 'k' => Some(KeyCode::K), 'l' => Some(KeyCode::L),
        'm' => Some(KeyCode::M), 'n' => Some(KeyCode::N), 'o' => Some(KeyCode::O),
        'p' => Some(KeyCode::P), 'q' => Some(KeyCode::Q), 'r' => Some(KeyCode::R),
        's' => Some(KeyCode::S), 't' => Some(KeyCode::T), 'u' => Some(KeyCode::U),
        'v' => Some(KeyCode::V), 'w' => Some(KeyCode::W), 'x' => Some(KeyCode::X),
        'y' => Some(KeyCode::Y), 'z' => Some(KeyCode::Z), _ => None,
    }
}
