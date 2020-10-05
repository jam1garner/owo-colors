use core::fmt;
use crate::{AnsiColors, DynColor, DynStylesColor};

#[allow(unused_imports)]
use crate::OwoColorize;

/// Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
/// or [`OwoColorize::on_color`](OwoColorize::on_color)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl DynColor for Rgb {
    fn get_fg(&self) -> DynStylesColor {
        let Rgb(r, g, b) = self;
        DynStylesColor::Rgb(*r, *g, *b)
    }

    fn get_bg(&self) -> DynStylesColor {
        self.get_fg()
    }

    fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Rgb(r, g, b) = self;
        write!(f, "\x1b[38;2;{};{};{}m", r, g, b)
    }

    fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Rgb(r, g, b) = self;
        write!(f, "\x1b[48;2;{};{};{}m", r, g, b)
    }
}

impl DynColor for str {
    fn get_fg(&self) -> DynStylesColor {
        let color: AnsiColors = self.into();
        color.get_fg()
    }

    fn get_bg(&self) -> DynStylesColor {
        let color: AnsiColors = self.into();
        color.get_bg()
    }
}

/// Implemented for drop-in replacement support for `colored`
impl<'a> From<&'a str> for AnsiColors {
    fn from(color: &'a str) -> Self {
        match color {
            "black" => AnsiColors::Black,
            "red" => AnsiColors::Red,
            "green" => AnsiColors::Green,
            "yellow" => AnsiColors::Yellow,
            "blue" => AnsiColors::Blue,
            "magenta" => AnsiColors::Magenta,
            "purple" => AnsiColors::Magenta,
            "cyan" => AnsiColors::Cyan,
            "white" => AnsiColors::White,
            "bright black" => AnsiColors::BrightBlack,
            "bright red" => AnsiColors::BrightRed,
            "bright green" => AnsiColors::BrightGreen,
            "bright yellow" => AnsiColors::BrightYellow,
            "bright blue" => AnsiColors::BrightBlue,
            "bright magenta" => AnsiColors::BrightMagenta,
            "bright cyan" => AnsiColors::BrightCyan,
            "bright white" => AnsiColors::BrightWhite,
            _ => AnsiColors::White,
        }
    }
}
