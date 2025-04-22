//! Contains cosmetic, color-related utilities.

/// Contains various color-related utilities for cosmetic customization.
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

/// Represents different colors. Used to color text or modify the appearance of
/// log headers.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
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

    BrightBlack = 10,
    BrightBlue = 11,
    BrightCyan = 12,
    BrightGreen = 13,
    BrightMagenta = 14,
    BrightRed = 15,
    BrightWhite = 16,
    BrightYellow = 17,
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

static BRIGHT_BLACK: &str = "\x1b[90m";
static BRIGHT_BLUE: &str = "\x1b[94m";
static BRIGHT_CYAN: &str = "\x1b[96m";
static BRIGHT_GREEN: &str = "\x1b[92m";
static BRIGHT_MAGENTA: &str = "\x1b[95m";
static BRIGHT_RED: &str = "\x1b[91m";
static BRIGHT_WHITE: &str = "\x1b[97m";
static BRIGHT_YELLOW: &str = "\x1b[93m";

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

        m.insert(Color::BrightBlack as i32, BRIGHT_BLACK);
        m.insert(Color::BrightBlue as i32, BRIGHT_BLUE);
        m.insert(Color::BrightCyan as i32, BRIGHT_CYAN);
        m.insert(Color::BrightGreen as i32, BRIGHT_GREEN);
        m.insert(Color::BrightMagenta as i32, BRIGHT_MAGENTA);
        m.insert(Color::BrightRed as i32, BRIGHT_RED);
        m.insert(Color::BrightWhite as i32, BRIGHT_WHITE);
        m.insert(Color::BrightYellow as i32, BRIGHT_YELLOW);
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
    return String::from(text)
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

            Color::BrightBlack => "BrightBlack",
            Color::BrightBlue => "BrightBlue",
            Color::BrightCyan => "BrightCyan",
            Color::BrightGreen => "BrightGreen",
            Color::BrightMagenta => "BrightMagenta",
            Color::BrightRed => "BrightRed",
            Color::BrightWhite => "BrightWhite",
            Color::BrightYellow => "BrightYellow",
        };
        return write!(f, "{}", level_str)
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

            10 => Ok(Color::BrightBlack),
            11 => Ok(Color::BrightBlue),
            12 => Ok(Color::BrightCyan),
            13 => Ok(Color::BrightGreen),
            14 => Ok(Color::BrightMagenta),
            15 => Ok(Color::BrightRed),
            16 => Ok(Color::BrightWhite),
            17 => Ok(Color::BrightYellow),
            _ => Err("Invalid value! Please provide a value in range 0-9."),
        }
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        match self {
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

            Color::BrightBlack => "BrightBlack",
            Color::BrightBlue => "BrightBlue",
            Color::BrightCyan => "BrightCyan",
            Color::BrightGreen => "BrightGreen",
            Color::BrightMagenta => "BrightMagenta",
            Color::BrightRed => "BrightRed",
            Color::BrightWhite => "BrightWhite",
            Color::BrightYellow => "BrightYellow",
        }
    }
}
