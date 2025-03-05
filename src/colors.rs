//! Contains cosmetic, color-related utilities.

/// Contains various color-related utilities for cosmetic customization.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
/// Represents different colors. Used to color text or set log header colors.
pub enum Color
{
    None = 0,
    Black = 1,
    #[default]
    Blue = 2,
    Cyan = 3,
    Green = 4,
    Gray = 5,
    Magenta = 6,
    Red = 7,
    White = 8,
    Yellow = 9,
}

static BLACK: &str = "\x1b[30m";
static BLUE: &str = "\x1b[34m";
static CYAN: &str = "\x1b[36m";
static GREEN: &str = "\x1b[32m";
static GRAY: &str = "\x1b[90m";
static MAGENTA: &str = "\x1b[35m";
static RED: &str = "\x1b[31m";
static WHITE: &str = "\x1b[37m";
static YELLOW: &str = "\x1b[33m";
pub(crate) static RESET: &str = "\x1b[0m";

lazy_static! {
    static ref COLOR_MAP: HashMap<i32, &'static str> =  {
        let mut m = HashMap::new();
        m.insert(Color::None as i32, "");
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

/// Colors given text based on `color` value using ANSII escape codes.
///
/// # Example
/// ```
/// # use prettylogger::colors::{Color, color_text};
/// let colored_text = color_text("a piece of text", Color::Red);
/// assert_eq!(colored_text, "\x1b[31ma piece of text\x1b[0m");
/// ```
pub fn color_text(text: &str, color: Color) -> String {
    if color != Color::None {
        return COLOR_MAP[&(color as i32)].to_string() + text + RESET;
    }
    return String::from(text);
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let level_str = match *self {
            Color::None => "None",
            Color:: Black => "Black",
            Color::Blue => "Blue",
            Color::Cyan => "Cyan",
            Color::Green => "Green",
            Color::Gray => "Gray",
            Color::Magenta => "Magenta",
            Color::Red => "Red",
            Color::White => "White",
            Color::Yellow => "Yellow",
        };
        write!(f, "{}", level_str)
    }
}

impl TryFrom<i32> for Color {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::None),
            1 => Ok(Color::Black),
            2 => Ok(Color::Blue),
            3 => Ok(Color::Cyan),
            4 => Ok(Color::Green),
            5 => Ok(Color::Gray),
            6 => Ok(Color::Magenta),
            7 => Ok(Color::Red),
            8 => Ok(Color::White),
            9 => Ok(Color::Yellow),
            _ => Err("Invalid value! Please provide a value in range 0-9."),
        }
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        match self {
            Color::None => "None",
            Color::Black => "Black",
            Color::Blue => "Blue",
            Color::Cyan => "Cyan",
            Color::Green => "Green",
            Color::Gray => "Gray",
            Color::Magenta => "Magenta",
            Color::Red => "Red",
            Color::White => "White",
            Color::Yellow => "Yellow",
        }
    }
}
