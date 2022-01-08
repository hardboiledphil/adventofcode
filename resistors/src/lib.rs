use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;
use std::convert::TryFrom;

#[repr(i8)]
#[derive(Debug, Eq, PartialEq, IntEnum, IntoEnumIterator, Copy, Clone)]
pub enum ResistorColor {
    Black  = 0,
    Brown  = 1,
    Red    = 2,
    Orange = 3,
    Yellow = 4,
    Green  = 5,
    Blue   = 6,
    Violet = 7,
    Grey   = 8,
    White  = 9,
}

impl TryFrom<usize> for ResistorColor {

    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResistorColor::Black),
            1 => Ok(ResistorColor::Brown),
            2 => Ok(ResistorColor::Red),
            3 => Ok(ResistorColor::Orange),
            4 => Ok(ResistorColor::Yellow),
            5 => Ok(ResistorColor::Green),
            6 => Ok(ResistorColor::Blue),
            7 => Ok(ResistorColor::Violet),
            8 => Ok(ResistorColor::Grey),
            9 => Ok(ResistorColor::White),
            _ => Err(())
        }
    }
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
   let result = ResistorColor::try_from(value);
    match result {
        Ok(color) => {
            match color {
                ResistorColor::Black  => String::from("Black"),
                ResistorColor::Blue   => String::from("Blue"),
                ResistorColor::Brown  => String::from("Brown"),
                ResistorColor::Green  => String::from("Green"),
                ResistorColor::Grey   => String::from("Grey"),
                ResistorColor::Orange => String::from("Orange"),
                ResistorColor::Red    => String::from("Red"),
                ResistorColor::Violet => String::from("Violet"),
                ResistorColor::White  => String::from("White"),
                ResistorColor::Yellow => String::from("Yellow"),
            }
        },
        Err(color) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}

#[cfg(test)]
mod tests {


    #[test]
    fn test_black() {
        assert_eq!(super::color_to_value(super::ResistorColor::Black), 0);
    }

    #[test]
    fn test_orange() {
        assert_eq!(super::color_to_value(super::ResistorColor::Orange), 3);
    }

    #[test]
    fn test_white() {
        assert_eq!(super::color_to_value(super::ResistorColor::White), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::value_to_color_string(2), String::from("Red"));
    }

    #[test]
    fn test_6() {
        assert_eq!(super::value_to_color_string(6), String::from("Blue"));
    }

    #[test]
    fn test_8() {
        assert_eq!(super::value_to_color_string(8), String::from("Grey"));
    }

    #[test]
    fn test_11_out_of_range() {
        assert_eq!(
            super::value_to_color_string(11), String::from("value out of range") );
    }

    #[test]
    fn test_all_colors() {
        use crate::ResistorColor::*;
        assert_eq!(
            super::colors(),
            vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
        );
    }
}
