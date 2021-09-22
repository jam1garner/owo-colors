//! Color types for used for being generic over the color
use crate::{BgColorDisplay, BgDynColorDisplay, FgColorDisplay, FgDynColorDisplay};
use core::fmt;

macro_rules! colors {
    ($(
        $color:ident $fg:literal $bg:literal
    ),* $(,)?) => {

        pub(crate) mod ansi_colors {
            use core::fmt;

            #[allow(unused_imports)]
            use crate::OwoColorize;

            /// Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color)
            /// or [`OwoColorize::on_color`](OwoColorize::on_color)
            #[allow(missing_docs)]
            #[derive(Copy, Clone, Debug, PartialEq)]
            pub enum AnsiColors {
                $(
                    $color,
                )*
            }

            impl crate::DynColor for AnsiColors {
                fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let color = match self {
                        $(
                            AnsiColors::$color => concat!("\x1b[", stringify!($fg), "m"),
                        )*
                    };

                    write!(f, "{}", color)
                }

                fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let color = match self {
                        $(
                            AnsiColors::$color => concat!("\x1b[", stringify!($bg), "m"),
                        )*
                    };

                    write!(f, "{}", color)
                }

                #[doc(hidden)]
                fn get_dyncolors_fg(&self) -> crate::DynColors {
                    crate::DynColors::Ansi(*self)
                }

                #[doc(hidden)]
                fn get_dyncolors_bg(&self) -> crate::DynColors {
                    crate::DynColors::Ansi(*self)
                }
            }
        }

        $(
            /// A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.
            pub struct $color;

            impl crate::Color for $color {
                const ANSI_FG: &'static str = concat!("\x1b[", stringify!($fg), "m");
                const ANSI_BG: &'static str = concat!("\x1b[", stringify!($bg), "m");

                #[doc(hidden)]
                fn into_dyncolors() -> crate::DynColors {
                    crate::DynColors::Ansi(ansi_colors::AnsiColors::$color)
                }
            }
        )*

    };
}

colors! {
    Black   30 40,
    Red     31 41,
    Green   32 42,
    Yellow  33 43,
    Blue    34 44,
    Magenta 35 45,
    Cyan    36 46,
    White   37 47,

    BrightBlack   90 100,
    BrightRed     91 101,
    BrightGreen   92 102,
    BrightYellow  93 103,
    BrightBlue    94 104,
    BrightMagenta 95 105,
    BrightCyan    96 106,
    BrightWhite   97 107,
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
                }
            }
        )*
    };
}

impl_fmt_for! {
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

macro_rules! impl_fmt_for_dyn {
    ($(($ty:ident, $trait:path, $fmt:ident)),* $(,)?) => {
        $(
            impl<'a, Color: crate::DynColor, T: $trait> $trait for $ty<'a, Color, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    (self.1).$fmt(f)?;
                    <T as $trait>::fmt(&self.0, f)?;
                    f.write_str("\x1b[0m")
                }
            }
        )*
    };
}

impl_fmt_for_dyn! {
    // Foreground
    (FgDynColorDisplay, fmt::Display,  fmt_ansi_fg),
    (FgDynColorDisplay, fmt::Debug,    fmt_ansi_fg),
    (FgDynColorDisplay, fmt::UpperHex, fmt_ansi_fg),
    (FgDynColorDisplay, fmt::LowerHex, fmt_ansi_fg),
    (FgDynColorDisplay, fmt::Binary,   fmt_ansi_fg),
    (FgDynColorDisplay, fmt::UpperExp, fmt_ansi_fg),
    (FgDynColorDisplay, fmt::LowerExp, fmt_ansi_fg),
    (FgDynColorDisplay, fmt::Octal,    fmt_ansi_fg),
    (FgDynColorDisplay, fmt::Pointer,  fmt_ansi_fg),

    // Background
    (BgDynColorDisplay, fmt::Display,  fmt_ansi_bg),
    (BgDynColorDisplay, fmt::Debug,    fmt_ansi_bg),
    (BgDynColorDisplay, fmt::UpperHex, fmt_ansi_bg),
    (BgDynColorDisplay, fmt::LowerHex, fmt_ansi_bg),
    (BgDynColorDisplay, fmt::Binary,   fmt_ansi_bg),
    (BgDynColorDisplay, fmt::UpperExp, fmt_ansi_bg),
    (BgDynColorDisplay, fmt::LowerExp, fmt_ansi_bg),
    (BgDynColorDisplay, fmt::Octal,    fmt_ansi_bg),
    (BgDynColorDisplay, fmt::Pointer,  fmt_ansi_bg),
}

/// CSS named colors. Not as widely supported as standard ANSI as it relies on 48bit color support.
///
/// Reference: https://www.w3schools.com/cssref/css_colors.asp
/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
pub mod css;
/// XTerm 256-bit colors. Not as widely supported as standard ANSI but contains 240 more colors.
pub mod xterm;

mod custom;

pub use custom::CustomColor;

pub(crate) mod dynamic;
