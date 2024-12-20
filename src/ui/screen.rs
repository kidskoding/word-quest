use std::sync::Mutex;

#[derive(Clone, Debug)]
pub enum Screen {
    Title,
    Game,
}

lazy_static::lazy_static! {
    static ref current_screen: Mutex<Screen> = Mutex::new(Screen::Game);
}

pub struct ScreenManager;

impl ScreenManager {
    #[allow(dead_code)]
    fn new() -> ScreenManager {
        ScreenManager
    }

    pub fn current_screen() -> Option<Screen> {
        match current_screen.lock() {
            Ok(guard) => Some(guard.clone()),
            Err(_) => None,
        }
    }
    pub fn switch_screen(screen: Screen) {
        let mut guard = current_screen.lock().unwrap();
        *guard = screen;
    }
}
