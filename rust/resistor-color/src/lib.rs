use int_enum::IntEnum;
use enum_iterator::{all, Sequence};
use std::fmt;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    White = 9,
    Grey = 8,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Convert a color into a numerical representation.
pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

/// Convert the value into a string representation of color.
pub fn value_to_color_string(value: usize) -> String {
    if let Ok(color) = ResistorColor::from_int(value as u8) {
        color.to_string()
    } else {
        "value out of range".to_string()
    }
}

/// Return a list of all the colors ordered by resistance.
pub fn colors() -> Vec<ResistorColor> {
    let mut all_colors = all::<ResistorColor>().collect::<Vec<_>>();

    all_colors.sort_by_key(|a| *a as usize );

    all_colors
}
