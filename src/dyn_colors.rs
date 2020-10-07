#[allow(unused_imports)]
use crate::{AnsiColors, XtermColors, FgDynColorDisplay, BgDynColorDisplay, DynColor, Rgb};
use core::fmt;

/// An enum describing runtime-configurable colors which can be displayed using [`FgDynColorDisplay`](FgDynColorDisplay)
/// or [`BgDynColorDisplay`](BgDynColorDisplay), allowing for multiple types of colors to be used
/// at runtime. 
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DynColors {
    Ansi(AnsiColors),
    Xterm(XtermColors),
    Rgb(u8, u8, u8),
}

impl DynColor for DynColors {
    fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynColors::Ansi(ansi) => ansi.fmt_ansi_fg(f),
            DynColors::Xterm(xterm) => xterm.fmt_ansi_fg(f),
            &DynColors::Rgb(r, g, b) => Rgb(r, g, b).fmt_ansi_fg(f),
        }
    }

    fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynColors::Ansi(ansi) => ansi.fmt_ansi_bg(f),
            DynColors::Xterm(xterm) => xterm.fmt_ansi_bg(f),
            &DynColors::Rgb(r, g, b) => Rgb(r, g, b).fmt_ansi_bg(f),
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
