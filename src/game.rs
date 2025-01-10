use macroquad::prelude::*;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use ::rand::Rng;
use once_cell::sync::Lazy;
use crate::ui;
use crate::json;
use crate::ui::screen::{Screen, ScreenManager};

static mut game_state: Lazy<GameState> = Lazy::new(|| GameState::new());

#[derive(Default)]
struct GameState {
    scoring: HashMap<char, i32>,
    words: Vec<char>,
    tiles: Vec<ui::tile::Tile>,
    current_word: String,
    round_score: u64,
    total_score: i32,
    words_db: HashSet<String>,
    words_remaining: u32,
    discards: u32,
    round: u32,
    guessed_words: HashSet<String>,
    filtered_words: HashSet<String>,
}

impl GameState {
    fn new() -> Self {
        let scoring = {
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

        let words: Vec<char> = scoring.keys().cloned().collect();

        let words_db = initialize_words_db()
            .expect("Failed to initialize words database: Could not load or deserialize the cache");

        let filtered_words = ('a'..='z').map(|c| c.to_string()).collect();

        GameState {
            scoring,
            words,
            tiles: Vec::new(),
            current_word: String::new(),
            round_score: 750,
            total_score: 0,
            words_db,
            words_remaining: 4,
            discards: 3,
            round: 1,
            guessed_words: HashSet::new(),
            filtered_words,
        }
    }
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

pub unsafe fn draw_screen() {
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

    let letter_dim = measure_text(&game_state.current_word, None, 40, 1.0);
    draw_text(
        &game_state.current_word,
        screen_width() / 2.0 + 50.0 + 475.0 / 2.0 - letter_dim.width / 2.0,
        screen_height() - 225.0 + 35.0,
        40.0,
        BLACK
    );
    
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

    if game_state.current_word.len() != 0 && (play_button.is_clicked() || is_key_pressed(KeyCode::Enter)) {
        let score = score_word().unwrap_or(0);
        clear_word();
        game_state.total_score += score;
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

unsafe fn draw_hud() {
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
    draw_text(
        &game_state.round_score.to_string(),
        125.0 + 190.0
            - measure_text(&game_state.round_score.to_string(), None, 40, 1.0).width / 2.0,
        250.0 - 50.0,
        60.0,
        WHITE
    );
    
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
            - measure_text("Round Score", None, 50, 1.0).width / 2.0,
        250.0 + 90.0
            - measure_text("Round Score", None, 50, 1.0).height / 2.0,
        50.0,
        BLACK
    );
    draw_text(
        &game_state.total_score.to_string(),
        125.0 + 400.0 / 2.0
            - measure_text(&game_state.total_score.to_string(), None, 120, 1.0).width / 2.0,
        250.0 + 190.0
            - measure_text(&game_state.total_score.to_string(), None, 120, 1.0).height / 2.0,
        120.0,
        BLACK
    );

    let coral_rgba = Color::new(255.0 / 255.0, 127.0 / 255.0, 80.0 / 255.0, 1.0);
    
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
    draw_text(
        &game_state.words_remaining.to_string(),
        175.0 + 125.0 / 2.0 - measure_text(&game_state.words_remaining.to_string(),
                                           None, 50, 1.0).width / 2.0,
        460.0 + 60.0,
        50.0,
        BLACK
    );

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

    let d = &game_state.discards.to_string();
    draw_text(
        &d,
        350.0 + 125.0 / 2.0 - measure_text(&d, None, 50, 1.0).width / 2.0,
        460.0 + 60.0,
        50.0,
        BLACK
    );

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
    draw_text(
        &game_state.round.to_string(),
        350.0 - 90.0 + 125.0 / 2.0 - measure_text(&game_state.round.to_string(), None, 50, 1.0).width / 2.0,
        560.0 + 60.0,
        50.0,
        WHITE
    );
}

unsafe fn draw_tiles() {
    let tiles = &mut game_state.tiles;
    if tiles.is_empty() {
        let mut start_y = 100.0;
        let mut rand_indexes: Vec<usize> = Vec::new();
        let mut rng = ::rand::thread_rng();

        while rand_indexes.len() < 12 {
            let rand_index = rng.gen_range(0..game_state.words.len());
            if !rand_indexes.contains(&rand_index) {
                rand_indexes.push(rand_index);
            }
        }

        let mut inc = 0;
        for r in 0..3 {
            let mut start_x = screen_width() / 2.0 + 50.0;
            for c in 0..4 {
                let tile = ui::tile::Tile::new(*game_state.words.get(rand_indexes[inc * 4 + c]).unwrap(),
                                               start_x, start_y);
                tiles.push(tile);
                start_x += 125.0;
            }
            start_y += 125.0;
            inc += 1;
        }
    }
    
    for tile in &game_state.tiles {
        tile.draw();
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

unsafe fn shuffle_tiles() {
    let mut tiles = &mut game_state.tiles;
    for i in 0..tiles.len() {
        let j = rand::gen_range(0, tiles.len());
        tiles.swap(i, j);
    }
    redraw(tiles);
}

unsafe fn discard_tiles() {
    if game_state.discards > 0 {
        game_state.tiles.clear();
        game_state.discards -= 1;
        clear_word();
    }
}

unsafe fn clear_word() {
    &game_state.current_word.clear();
}

unsafe fn score_word() -> Option<i32> {
    if game_state.words_remaining > 0 {
        let word = &game_state.current_word;
        let set: HashSet<_> = word.chars().collect();
        if game_state.words_db.contains(word) && !game_state.guessed_words.contains(word)
            && set.len() != 1 {
            let mut score = 0;
            for char in word.chars() {
                score += game_state.scoring.get(&char).unwrap_or(&0);
            }
            score *= word.len() as i32;
            game_state.guessed_words.remove(word);
            game_state.words_remaining -= 1;
            return Some(score);
        }
        game_state.words_remaining -= 1;
    }
    None
}

unsafe fn update() {
    for tile in &mut game_state.tiles {
        let tile_letter = tile.get_letter();
        if let Some(key_code) = char_to_key_code(tile_letter) {
            if is_key_pressed(key_code) || tile.is_clicked() {
                game_state.current_word.push(tile_letter);
            }
        }
    }

    if game_state.total_score >= game_state.round_score as i32 {
        ScreenManager::switch_screen(Screen::RoundWinScreen);
        game_state.round_score += 100;
        discard_tiles();
        game_state.discards = 3;
        game_state.words_remaining = 4;
        if game_state.round == 5 {
            ScreenManager::switch_screen(Screen::RoundWinScreen);
        } else {
            game_state.round += 1;
        }
        game_state.guessed_words = HashSet::new();
        game_state.total_score = 0;
    }

    if game_state.words_remaining == 0 {
        ScreenManager::switch_screen(Screen::LoseScreen);
        game_state.words_remaining = 4;
        discard_tiles();
        game_state.discards = 3;
        game_state.total_score = 0;
        game_state.round_score = 750;
        game_state.round = 1;
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
