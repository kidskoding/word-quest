use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use lazy_static::lazy_static;
use ::rand::Rng;

use crate::ui;
use crate::json;
use crate::ui::screen::{Screen, ScreenManager};

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
    
    static ref round_score: Mutex<u64> = Mutex::new(750);
    pub static ref total_score: Mutex<i32> = Mutex::new(0);
    
    static ref words_db: HashSet<String> = initialize_words_db()
        .expect("Failed to initialize words database: Could not load or deserialize the cache");
    static ref words_remaining: Mutex<u32> = Mutex::new(4);
    static ref discards: Mutex<u32> = Mutex::new(3);
    static ref round: Mutex<u32> = Mutex::new(1);
    
    static ref guessed_words: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    static ref filtered_words: HashSet<String> = {
        let mut set = HashSet::new();
        for c in 'a'..='z' {
            set.insert(c.to_string());
        }
        set
    };
}

fn initialize_words_db() -> Result<HashSet<String>, String> {
    let cache_file = "src/words_cache.json";

    if let Ok(cache_contents) = fs::read_to_string(&cache_file) {
        if let Ok(cached_words) = serde_json::from_str(&cache_contents) {
            return Ok(cached_words);
        }
    }

    let db: HashSet<_> = HashSet::from_iter(json::get_available_words());

    match serde_json::to_string(&db) {
        Ok(serialized) => {
            match File::create(cache_file) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(serialized.as_bytes()) {
                        return Err(format!("Failed to write to cache file: {}", e));
                    }
                },
                Err(e) => return Err(format!("Failed to create or overwrite cache file: {}", e)),
            }
        },
        Err(e) => return Err(format!("Failed to serialize empty words database: {}", e)),
    }

    Ok(db)
}

pub fn draw_screen() {
    clear_background(DARKGRAY);
    
    draw_hud();
    
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
        let score = score_word().unwrap_or(0);
        clear_word();
        if let Ok(mut guard) = total_score.lock() {
            *guard += score;
        }
    }

    let x_button = ui::button::Button::new(
        screen_width() - 165.0,
        screen_height() - 150.0,
        50.0,
        50.0,
        RED,
        "X".to_string(),
        40.0
    );
    x_button.draw();
    if x_button.is_clicked() || is_key_pressed(KeyCode::Backspace) {
        clear_word();
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
    
    update();
}

fn draw_hud() {
    draw_rectangle(
        100.0,
        50.0,
        450.0,
        600.0,
        WHITE
    );
    
    draw_rectangle(
        125.0,
        75.0,
        400.0,
        150.0,
        BLACK
    );
    draw_text(
        "Score at least",
        125.0 + 300.0 / 2.0
            - measure_text("Score at least", None, 40, 1.0).width / 2.0,
        190.0 - 50.0,
        55.0,
        WHITE
    );
    if let Ok(guard) = round_score.lock() {
        draw_text(
            &*guard.to_string(),
            125.0 + 190.0
                - measure_text(&*guard.to_string(), None, 40, 1.0).width / 2.0,
            250.0 - 50.0,
            60.0,
            WHITE
        );    
    }
    
    draw_rectangle(
        125.0,
        250.0,
        400.0,
        200.0,
        LIME
    );
    draw_text(
        "Round Score",
        125.0 + 400.0 / 2.0
            - measure_text("Total Score", None, 30, 1.0).width / 2.0,
        250.0 + 50.0
            - measure_text("Total Score", None, 30, 1.0).height / 2.0,
        30.0,
        BLACK
    );
    if let Ok(guard) = total_score.lock() {
        draw_text(
            &guard.to_string(),
            125.0 + 400.0 / 2.0
                - measure_text(&guard.to_string(), None, 80, 1.0).width / 2.0,
            250.0 + 120.0
                - measure_text(&guard.to_string(), None, 80, 1.0).height / 2.0,
            80.0,
            BLACK
        );
    } else {
        println!("Error: Failed to get total score lock");
    }

    draw_rectangle(
        175.0,
        360.0,
        125.0,
        75.0,
        SKYBLUE
    );

    let coral_rgba = Color::new(255.0 / 255.0, 127.0 / 255.0, 80.0 / 255.0, 1.0);
    draw_rectangle(
        350.0,
        360.0,
        125.0,
        75.0,
        coral_rgba
    );

    let rect1_center_x = 175.0 + 125.0 / 2.0;
    let rect2_center_x = 350.0 + 125.0 / 2.0;
    let center_x = (rect1_center_x + rect2_center_x) / 2.0;
    let center_y = 360.0 + 75.0 / 2.0;
    let cross_size = 20.0;

    draw_line(
        center_x - cross_size / 2.0, center_y - cross_size / 2.0,
        center_x + cross_size / 2.0, center_y + cross_size / 2.0,
        5.0, BLACK
    );
    draw_line(
        center_x + cross_size / 2.0, center_y - cross_size / 2.0,
        center_x - cross_size / 2.0, center_y + cross_size / 2.0,
        5.0, BLACK
    );

    draw_rectangle(
        175.0,
        460.0,
        125.0,
        75.0,
        SKYBLUE
    );
    draw_text(
        "Words",
        175.0 + 125.0 / 2.0 - measure_text("Words", None, 30, 1.0).width / 2.0,
        460.0 + 25.0,
        30.0,
        BLACK
    );
    if let Ok(guard) = words_remaining.lock() {
        let w = &*guard.to_string();
        draw_text(
            &w,
            175.0 + 125.0 / 2.0 - measure_text(&w, None, 50, 1.0).width / 2.0,
            460.0 + 60.0,
            50.0,
            BLACK
        );
    }

    draw_rectangle(
        350.0,
        460.0,
        125.0,
        75.0,
        coral_rgba
    );
    draw_text(
        "Discards",
        350.0 + 125.0 / 2.0 - measure_text("Discards", None, 30, 1.0).width / 2.0,
        460.0 + 25.0,
        30.0,
        BLACK
    );

    if let Ok(guard) = discards.lock() {
        let d = &*guard.to_string();
        draw_text(
            &d,
            350.0 + 125.0 / 2.0 - measure_text(&d, None, 50, 1.0).width / 2.0,
            460.0 + 60.0,
            50.0,
            BLACK
        );
    } else {
        draw_text(
            "0",
            350.0 + 125.0 / 2.0 - measure_text("0", None, 50, 1.0).width / 2.0,
            460.0 + 60.0,
            50.0,
            BLACK
        );
    }

    draw_rectangle(
        350.0 - 90.0,
        560.0,
        125.0,
        75.0,
        BLACK
    );
    draw_text(
        "Round",
        350.0 - 90.0 + 125.0 / 2.0 - measure_text("Round", None, 30, 1.0).width / 2.0,
        560.0 + 25.0,
        30.0,
        WHITE
    );
    if let Ok(guard) = round.lock() {
        draw_text(
            &*guard.to_string(),
            350.0 - 90.0 + 125.0 / 2.0 - measure_text(&*guard.to_string(), None, 50, 1.0).width / 2.0,
            560.0 + 60.0,
            50.0,
            WHITE
        );    
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
    if let Ok(mut guard1) = discards.lock() {
        if *guard1 > 0 {
            if let Ok(mut guard2) = tiles.lock() {
                guard2.clear();
            } else {
                println!("Error: Failed to get tiles lock");
            }
            *guard1 -= 1;
            clear_word();
        }
    }
}

fn clear_word() {
    if let Ok(mut word) = current_word.lock() {
        word.clear();
    } else {
        println!("Error: Failed to get current word lock");
    }
}

fn score_word() -> Option<i32> {
    let mut temp = words_remaining.lock().unwrap();
    if *temp > 0 {
        let word = current_word.lock().unwrap();
        let set: HashSet<_> = word.chars().collect();
        if words_db.contains(&word.clone()) && !guessed_words.lock().unwrap().contains(&word.clone())
            && set.len() != 1 {
            let score = word.chars().map(|c| scoring.get(&c).unwrap_or(&0)).sum::<i32>()
                * word.len() as i32;
            guessed_words.lock().unwrap().insert(word.clone());
            *temp -= 1;
            return Some(score);
        }
        *temp -= 1;
    }
    None
}

fn update() {
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
    
    if let Ok(mut guard) = total_score.lock() {
        if let Ok(mut rs) = round_score.lock() {
            if *guard >= *rs as i32 {
                ScreenManager::switch_screen(Screen::RoundWinScreen);
                *rs *= 2;
                if let Ok(mut d) = discards.lock() {
                    *d = 3;
                }
                if let Ok(mut w) = words_remaining.lock() {
                    *w = 4;
                }
                if let Ok(mut rounds) = round.lock() {
                    if *rounds == 5 {
                        ScreenManager::switch_screen(Screen::WinScreen);
                    } else {
                        *rounds = *rounds + 1;
                    }
                }
                *guard = 0;
            }
        }
    }
    
    if let Ok(mut guard) = words_remaining.lock() {
        if *guard == 0 {
            ScreenManager::switch_screen(Screen::LoseScreen);
            *guard = 4;
            if let Ok(mut guard) = discards.lock() {
                *guard = 3;
            }
            if let Ok(mut total) = total_score.lock() {
                *total = 0;
            }
        }
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
