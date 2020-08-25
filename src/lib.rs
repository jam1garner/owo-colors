#![cfg_attr(not(test), no_std)]
use core::fmt;
use core::marker::PhantomData;

pub trait Color {
    const ANSI_FG: &'static str;
    const ANSI_BG: &'static str;
}

#[repr(transparent)]
pub struct FgColorDisplay<'a, C: Color, T>(&'a T, PhantomData<C>);

#[repr(transparent)]
pub struct BgColorDisplay<'a, C: Color, T>(&'a T, PhantomData<C>);

#[repr(transparent)]
pub struct BoldDisplay<'a, T>(&'a T);

macro_rules! color_methods {
    ($(
        $color:ident $fg_method:ident $bg_method:ident
    ),* $(,)?) => {
        $(
            fn $fg_method<'a>(&'a self) -> FgColorDisplay<'a, colors::$color, Self> {
                FgColorDisplay(self, PhantomData)
            }
            
            fn $bg_method<'a>(&'a self) -> BgColorDisplay<'a, colors::$color, Self> {
                BgColorDisplay(self, PhantomData)
            }
         )*
    };
}

pub trait OwoColorize: fmt::Display + Sized {
    fn fg<'a, C: Color>(&'a self) -> FgColorDisplay<'a, C, Self> {
        FgColorDisplay(self, PhantomData)
    }
 
    fn bg<'a, C: Color>(&'a self) -> BgColorDisplay<'a, C, Self> {
        BgColorDisplay(self, PhantomData)
    }

    fn bold<'a>(&'a self) -> BoldDisplay<'a, Self> {
        BoldDisplay(&self)
    }

    color_methods!{
        Black    black    on_black,
        Red      red      on_red,
        Green    green    on_green,
        Yellow   yellow   on_yellow,
        Blue     blue     on_blue,
        Magenta  magenta  on_magenta,
        Cyan     cyan     on_cyan,
        White    white    on_white,
    }
}

impl<D: fmt::Display> OwoColorize for D {}

macro_rules! colors {
    ($(
        $color:ident $code:literal
    ),*) => {
        use crate::Color;

        $(
            pub struct $color;

            impl Color for $color {
                const ANSI_FG: &'static str = concat!("\x1b[3", stringify!($code), "m");
                const ANSI_BG: &'static str = concat!("\x1b[4", stringify!($code), "m");
            }
        )*
    };
}

pub mod colors {
    colors!{
        Black   0,
        Red     1,
        Green   2,
        Yellow  3,
        Blue    4,
        Magenta 5,
        Cyan    6,
        White   7
    }
}

macro_rules! impl_fmt_for {
    ($(($ty:ident, $trait:path, $const:ident)),* $(,)?) => {
        $(
            impl<'a, Color: crate::Color, T: $trait> $trait for $ty<'a, Color, T> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str(Color::$const)?;
                    <_ as $trait>::fmt(&self.0, f)?;
                    f.write_str("\x1b[0m")
                }
            }
        )*
    };
}

impl_fmt_for!{
    (FgColorDisplay, fmt::Display,  ANSI_FG),
    (FgColorDisplay, fmt::Debug,    ANSI_FG),
    (FgColorDisplay, fmt::UpperHex, ANSI_FG),
    (FgColorDisplay, fmt::LowerHex, ANSI_FG),
    (FgColorDisplay, fmt::Binary,   ANSI_FG),
    (FgColorDisplay, fmt::UpperExp, ANSI_FG),
    (FgColorDisplay, fmt::LowerExp, ANSI_FG),
    (FgColorDisplay, fmt::Octal,    ANSI_FG),
    (FgColorDisplay, fmt::Pointer,  ANSI_FG),
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

pub mod prelude {
    pub use crate::OwoColorize;
    pub use crate::colors::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_fg() {
        assert_eq!(
            "test".fg::<Black>().to_string(),
            "\x1b[30mtest\x1b[0m"
        );
        assert_eq!(
            "blah blah".red().to_string(),
            "\x1b[31mblah blah\x1b[0m"
        );
    }

    #[test]
    fn test_bg() {
        assert_eq!(
            "test".bg::<Black>().to_string(),
            "\x1b[40mtest\x1b[0m"
        );
        assert_eq!(
            "blah blah".on_red().to_string(),
            "\x1b[41mblah blah\x1b[0m"
        );
    }

    #[test]
    fn test_hex() {
        assert_eq!(
            format!("{:08X}", 0xa.red()),
            "\x1b[31m0000000A\x1b[0m"
        );
    }
}
