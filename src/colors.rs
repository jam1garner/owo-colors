use core::fmt;
use crate::{FgColorDisplay, BgColorDisplay};

macro_rules! colors {
    ($(
        $color:ident $fg:literal $bg:literal
    ),* $(,)?) => {
        use crate::Color;

        $(
            /// A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.
            pub struct $color;

            impl Color for $color {
                const ANSI_FG: &'static str = concat!("\x1b[", $fg, "m");
                const ANSI_BG: &'static str = concat!("\x1b[", $bg, "m");
            }
        )*
    };
}

colors!{
    Black   "30" "40",
    Red     "31" "41",
    Green   "32" "42",
    Yellow  "33" "43",
    Blue    "34" "44",
    Magenta "35" "45",
    Cyan    "36" "46",
    White   "37" "47",

    BrightBlack   "90" "100",
    BrightRed     "91" "101",
    BrightGreen   "92" "102",
    BrightYellow  "93" "103",
    BrightBlue    "94" "104",
    BrightMagenta "95" "105",
    BrightCyan    "96" "106",
    BrightWhite   "97" "107",
}

macro_rules! impl_fmt_for {
    ($(($ty:ident, $trait:path, $const:ident)),* $(,)?) => {
        $(
            impl<'a, Color: crate::Color, T: $trait> $trait for $ty<'a, Color, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str(Color::$const)?;
                    <T as $trait>::fmt(&self.0, f)?;
                    f.write_str("\x1b[0m")
                    //write!(f, "{}{}\x1b[0m", Color::$const, self.0)
                }
            }
        )*
    };
}

impl_fmt_for!{
    // Foreground
    (FgColorDisplay, fmt::Display,  ANSI_FG),
    (FgColorDisplay, fmt::Debug,    ANSI_FG),
    (FgColorDisplay, fmt::UpperHex, ANSI_FG),
    (FgColorDisplay, fmt::LowerHex, ANSI_FG),
    (FgColorDisplay, fmt::Binary,   ANSI_FG),
    (FgColorDisplay, fmt::UpperExp, ANSI_FG),
    (FgColorDisplay, fmt::LowerExp, ANSI_FG),
    (FgColorDisplay, fmt::Octal,    ANSI_FG),
    (FgColorDisplay, fmt::Pointer,  ANSI_FG),
    // Background
    (BgColorDisplay, fmt::Display,  ANSI_BG),
    (BgColorDisplay, fmt::Debug,    ANSI_BG),
    (BgColorDisplay, fmt::UpperHex, ANSI_BG),
    (BgColorDisplay, fmt::LowerHex, ANSI_BG),
    (BgColorDisplay, fmt::Binary,   ANSI_BG),
    (BgColorDisplay, fmt::UpperExp, ANSI_BG),
    (BgColorDisplay, fmt::LowerExp, ANSI_BG),
    (BgColorDisplay, fmt::Octal,    ANSI_BG),
    (BgColorDisplay, fmt::Pointer,  ANSI_BG),
}
