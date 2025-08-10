//! Contains cosmetic, color-related utilities.

/// Contains various color-related utilities for cosmetic customization.
use std::{
    collections::HashMap,
    fmt::{
        Display,
        Formatter
    }, sync::LazyLock,
};

use serde::{
    Serialize,
    Deserialize
};

/// Represents different colors. Used to color text or modify the appearance of
/// log headers.
///
/// # Examples
///
/// Using a standard `Color` to customize log header appearance:
/// ```rust
/// # use prettylogger::{
/// #     Logger,
/// #     colors::Color,
/// # };
/// let mut logger = Logger::default();
///
/// logger.formatter.lock().unwrap().set_debug_color(Color::Gray);
/// logger.formatter.lock().unwrap().set_info_color(Color::Green);
/// logger.formatter.lock().unwrap().set_warning_color(Color::Yellow);
/// logger.formatter.lock().unwrap().set_error_color(Color::Red);
/// logger.formatter.lock().unwrap().set_fatal_color(Color::Magenta);
/// ```
///
/// Using a custom `Color` to customize log header appearance:
/// ```rust
/// # use prettylogger::{
/// #     Logger,
/// #     colors::Color,
/// # };
/// let mut logger = Logger::default();
///
/// // Set a **bold white** color
/// logger.formatter.lock().unwrap()
///     .set_debug_color(Color::Custom(String::from("\x1b[97m")));
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
    Serialize, Deserialize)]
#[repr(i32)]
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

    Custom(String) = 10,
}

const BLACK: &str = "\x1b[30m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const GRAY: &str = "\x1b[90m";
const MAGENTA: &str = "\x1b[35m";
const RED: &str = "\x1b[31m";
const WHITE: &str = "\x1b[37m";
const YELLOW: &str = "\x1b[33m";

pub(crate) static RESET: &str = "\x1b[0m";

static COLOR_MAP: LazyLock<HashMap<i32, &str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Color::None.into(), "");
    m.insert(Color::Black.into(), BLACK);
    m.insert(Color::Blue.into(), BLUE);
    m.insert(Color::Cyan.into(), CYAN);
    m.insert(Color::Green.into(), GREEN);
    m.insert(Color::Gray.into(), GRAY);
    m.insert(Color::Magenta.into(), MAGENTA);
    m.insert(Color::Red.into(), RED);
    m.insert(Color::White.into(), WHITE);
    m.insert(Color::Yellow.into(), YELLOW);
    m
});

/// Colors given text based on `color` value using ANSII escape codes.
///
/// # Examples
///
/// Using a `Color` enum to color text:
/// ```
/// # use prettylogger::colors::{Color, color_text};
/// let colored_text = color_text("some text", Color::Red);
/// # assert_eq!(colored_text, "\x1b[31msome text\x1b[0m");
/// ```
///
/// Using a custom `Color` to color text:
/// ```
/// # use prettylogger::colors::{Color, color_text};
/// let colored_text = color_text("some text",
///     Color::Custom(String::from("\x1b[97m")));
/// # assert_eq!(colored_text, "\x1b[97msome text\x1b[0m");
/// ```
pub fn color_text(text: &str, color: Color) -> String {
    match color {
        Color::Custom(s) => {
            s + text + RESET
        },
        _ => {
            if color != Color::None {
                COLOR_MAP[&(color.into())].to_string() + text + RESET
            }
            else{
                String::from(text)
            }
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let level_str = match self {
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

            Color::Custom(str) => &format!("'{str}'")
        };
        write!(f, "{level_str}")
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
            18 => {Ok(Color::Custom(String::new()))}
            _ => Err("Invalid value! Please provide a value in range 0-9."),
        }
    }
}

impl From<Color> for i32 {
    fn from(color: Color) -> Self {
        match color {
            Color::None => 0,
            Color::Black => 1,
            Color::Blue => 2,
            Color::Cyan => 3,
            Color::Green => 4,
            Color::Gray => 5,
            Color::Magenta => 6,
            Color::Red => 7,
            Color::White => 8,
            Color::Yellow => 9,
            Color::Custom(_) => 10,
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
            Color::Custom(str) => str.as_str(),
        }
    }
}
