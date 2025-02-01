use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

static BLACK: &str = "\x1b[30m";
static BLUE: &str = "\x1b[34m";
static CYAN: &str = "\x1b[36m";
static GREEN: &str = "\x1b[32m";
static GRAY: &str = "\x1b[90m";
static MAGENTA: &str = "\x1b[35m";
static RED: &str = "\x1b[31m";
static WHITE: &str = "\x1b[37m";
static YELLOW: &str = "\x1b[33m";
static RESET: &str = "\x1b[0m";

#[derive(Clone)]
pub enum Color
{
    Black = 1,
    Blue = 2,
    Cyan = 3,
    Green = 4,
    Gray = 5,
    Magenta = 6,
    Red = 7,
    White = 8,
    Yellow = 9,
}

lazy_static! {
    static ref COLOR_MAP: HashMap<i32, &'static str> =  {
        let mut m = HashMap::new();
        m.insert(0, RESET);
        m.insert(Color::Black as i32, BLACK);
        m.insert(Color::Blue as i32, BLUE);
        m.insert(Color::Cyan as i32, CYAN);
        m.insert(Color::Green as i32, GREEN);
        m.insert(Color::Gray as i32, GRAY);
        m.insert(Color::Magenta as i32, MAGENTA);
        m.insert(Color::Red as i32, RED);
        m.insert(Color::White as i32, WHITE);
        m.insert(Color::Yellow as i32, YELLOW);
        return m;
    };
}

lazy_static! {
    pub static ref USE_COLOR: Mutex<bool> = Mutex::new(true);
}

fn get_color_code(color: Color) -> String {
    let key = color as i32;
    if COLOR_MAP.contains_key(&key) {
        return COLOR_MAP[&key].to_string();
    }
    else {
        return RESET.to_string();
    }
}

pub fn colorify(text: &str, color: Color) -> String {
    let use_color = USE_COLOR.lock().unwrap();
    if *use_color {
        return get_color_code(color) + text + RESET;
    }
    else {
        return text.to_string();
    }
}
