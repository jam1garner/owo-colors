use crate::{AnsiColors, DynColor};
use core::fmt;

#[allow(unused_imports)]
use crate::OwoColorize;

/// Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
/// or [`OwoColorize::on_color`](OwoColorize::on_color)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl DynColor for Rgb {
    fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "\x1b[38;2;{};{};{}m", r, g, b)
    }

    fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "\x1b[48;2;{};{};{}m", r, g, b)
    }

    fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "38;2;{};{};{}", r, g, b)
    }

    fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "48;2;{};{};{}", r, g, b)
    }

    #[doc(hidden)]
    fn get_dyncolors_fg(&self) -> crate::DynColors {
        let Self(r, g, b) = self;
        crate::DynColors::Rgb(*r, *g, *b)
    }

    #[doc(hidden)]
    fn get_dyncolors_bg(&self) -> crate::DynColors {
        self.get_dyncolors_fg()
    }
}

impl DynColor for str {
    fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color: AnsiColors = self.into();
        color.fmt_ansi_fg(f)
    }

    fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color: AnsiColors = self.into();
        color.fmt_ansi_bg(f)
    }

    fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color: AnsiColors = self.into();
        color.fmt_raw_ansi_fg(f)
    }

    fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color: AnsiColors = self.into();
        color.fmt_raw_ansi_bg(f)
    }

    #[doc(hidden)]
    fn get_dyncolors_fg(&self) -> crate::DynColors {
        crate::DynColors::Ansi(self.into())
    }

    #[doc(hidden)]
    fn get_dyncolors_bg(&self) -> crate::DynColors {
        crate::DynColors::Ansi(self.into())
    }
}

/// Implemented for drop-in replacement support for `colored`
impl<'a> From<&'a str> for AnsiColors {
    fn from(color: &'a str) -> Self {
        #[allow(clippy::match_same_arms)] // defaults to white color
        match color {
            "black" => Self::Black,
            "red" => Self::Red,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "magenta" | "purple" => Self::Magenta,
            "cyan" => Self::Cyan,
            "white" => Self::White,
            "bright black" => Self::BrightBlack,
            "bright red" => Self::BrightRed,
            "bright green" => Self::BrightGreen,
            "bright yellow" => Self::BrightYellow,
            "bright blue" => Self::BrightBlue,
            "bright magenta" => Self::BrightMagenta,
            "bright cyan" => Self::BrightCyan,
            "bright white" => Self::BrightWhite,
            _ => Self::White,
        }
    }
}
