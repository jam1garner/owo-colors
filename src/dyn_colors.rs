#[allow(unused_imports)]
use crate::{AnsiColors, XtermColors, FgDynColorDisplay, BgDynColorDisplay, DynColor, Rgb};
use crate::DynStylesColor;
use core::fmt;

/// An enum describing runtime-configurable colors which can be displayed using [`FgDynColorDisplay`](FgDynColorDisplay)
/// or [`BgDynColorDisplay`](BgDynColorDisplay), allowing for multiple types of colors to be used
/// at runtime. 
#[derive(Copy, Clone, PartialEq)]
pub enum DynColors {
    Ansi(AnsiColors),
    Xterm(XtermColors),
    Rgb(u8, u8, u8),
}

macro_rules! impl_fmt {
    ($($trait:path),* $(,)?) => {
        $(
            impl $trait for DynColors {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        Self::Ansi(ansi) => <AnsiColors as $trait>::fmt(&ansi, f),
                        Self::Xterm(xterm) => <XtermColors as $trait>::fmt(&xterm, f), 
                        Self::Rgb(r, g, b) => <Rgb as $trait>::fmt(&Rgb(*r, *g, *b), f),
                    }
                }
            }
        )*
    };
}

impl_fmt! {
    fmt::Display,
    fmt::Debug,
    fmt::UpperHex,
    fmt::LowerHex,
    fmt::Binary,
    fmt::UpperExp,
    fmt::LowerExp,
    fmt::Octal,
    fmt::Pointer,
}

impl DynColor for DynColors {
    fn get_fg(&self) -> DynStylesColor {
        match self {
            DynColors::Ansi(ansi) => ansi.get_fg(),
            DynColors::Xterm(xterm) => xterm.get_fg(),
            &DynColors::Rgb(r, g, b) => Rgb(r, g, b).get_fg(),
        }
    }

    fn get_bg(&self) -> DynStylesColor {
        match self {
            DynColors::Ansi(ansi) => ansi.get_bg(),
            DynColors::Xterm(xterm) => xterm.get_bg(),
            &DynColors::Rgb(r, g, b) => Rgb(r, g, b).get_bg(),
        }
    }
}

#[derive(Debug)]
pub struct ParseColorError;

impl core::str::FromStr for DynColors {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(0).ok_or(ParseColorError)? == '#' {
            match s.len() {
                4 => {
                    // TODO
                    Err(ParseColorError)
                },
                7 => {
                    Ok(Self::Rgb(
                        u8::from_str_radix(&s[1..3], 16).or(Err(ParseColorError))?,
                        u8::from_str_radix(&s[3..5], 16).or(Err(ParseColorError))?,
                        u8::from_str_radix(&s[5..7], 16).or(Err(ParseColorError))?,
                    ))
                }
                _ => Err(ParseColorError),
            }
        }
        else {
            let ansi = match s {
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
                _ => return Err(ParseColorError)
            };

            Ok(Self::Ansi(ansi))
        }
    } 
}
