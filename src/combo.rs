use crate::colors;
use crate::{BgColorDisplay, Color, FgColorDisplay};

use core::fmt;
use core::marker::PhantomData;

/// A wrapper type which applies both a foreground and background color
pub struct ComboColorDisplay<'a, Fg: Color, Bg: Color, T>(&'a T, PhantomData<(Fg, Bg)>);

macro_rules! impl_fmt_for_combo {
    ($($trait:path),* $(,)?) => {
        $(
            impl<'a, Fg: Color, Bg: Color, T: $trait> $trait for ComboColorDisplay<'a, Fg, Bg, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("\x1b[")?;
                    f.write_str(Fg::RAW_ANSI_FG)?;
                    f.write_str(";")?;
                    f.write_str(Bg::RAW_ANSI_BG)?;
                    f.write_str("m")?;
                    <T as $trait>::fmt(&self.0, f)?;
                    f.write_str("\x1b[0m")
                }
            }
        )*
    };
}

impl_fmt_for_combo! {
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

/// implement specialized color methods for FgColorDisplay BgColorDisplay, ComboColorDisplay
macro_rules! color_methods {
    ($(
        #[$fg_meta:meta] #[$bg_meta:meta] $color:ident $fg_method:ident $bg_method:ident
    ),* $(,)?) => {
        const _: () = (); // workaround for syntax highlighting bug

        impl<'a, Fg, T> FgColorDisplay<'a, Fg, T>
        where
            Fg: Color,
        {
            $(
                #[$fg_meta]
                #[inline(always)]
                pub fn $fg_method(self) -> FgColorDisplay<'a, colors::$color, T> {
                    FgColorDisplay(self.0, PhantomData)
                }

                #[$bg_meta]
                #[inline(always)]
                pub fn $bg_method(self) -> ComboColorDisplay<'a, Fg, colors::$color, T> {
                    ComboColorDisplay(self.0, PhantomData)
                }
             )*
        }

        const _: () = (); // workaround for syntax highlighting bug

        impl<'a, Bg, T> BgColorDisplay<'a, Bg, T>
        where
            Bg: Color,
        {
            $(
                #[$bg_meta]
                #[inline(always)]
                pub fn $bg_method(self) -> BgColorDisplay<'a, colors::$color, T> {
                    BgColorDisplay(self.0, PhantomData)
                }

                #[$fg_meta]
                #[inline(always)]
                pub fn $fg_method(self) -> ComboColorDisplay<'a, colors::$color, Bg, T> {
                    ComboColorDisplay(self.0, PhantomData)
                }
             )*
        }

        const _: () = (); // workaround for syntax highlighting bug

        impl<'a, Fg, Bg, T> ComboColorDisplay<'a, Fg, Bg, T>
        where
            Fg: Color,
            Bg: Color,
        {
            $(
                #[$bg_meta]
                #[inline(always)]
                pub fn $bg_method(self) -> ComboColorDisplay<'a, Fg, colors::$color, T> {
                    ComboColorDisplay(self.0, PhantomData)
                }

                #[$fg_meta]
                #[inline(always)]
                pub fn $fg_method(self) -> ComboColorDisplay<'a, colors::$color, Bg, T> {
                    ComboColorDisplay(self.0, PhantomData)
                }
             )*
        }
    };
}

const _: () = (); // workaround for syntax highlighting bug

color_methods! {
    /// Change the foreground color to black
    /// Change the background color to black
    Black    black    on_black,
    /// Change the foreground color to red
    /// Change the background color to red
    Red      red      on_red,
    /// Change the foreground color to green
    /// Change the background color to green
    Green    green    on_green,
    /// Change the foreground color to yellow
    /// Change the background color to yellow
    Yellow   yellow   on_yellow,
    /// Change the foreground color to blue
    /// Change the background color to blue
    Blue     blue     on_blue,
    /// Change the foreground color to magenta
    /// Change the background color to magenta
    Magenta  magenta  on_magenta,
    /// Change the foreground color to purple
    /// Change the background color to purple
    Magenta  purple   on_purple,
    /// Change the foreground color to cyan
    /// Change the background color to cyan
    Cyan     cyan     on_cyan,
    /// Change the foreground color to white
    /// Change the background color to white
    White    white    on_white,

    /// Change the foreground color to bright black
    /// Change the background color to bright black
    BrightBlack    bright_black    on_bright_black,
    /// Change the foreground color to bright red
    /// Change the background color to bright red
    BrightRed      bright_red      on_bright_red,
    /// Change the foreground color to bright green
    /// Change the background color to bright green
    BrightGreen    bright_green    on_bright_green,
    /// Change the foreground color to bright yellow
    /// Change the background color to bright yellow
    BrightYellow   bright_yellow   on_bright_yellow,
    /// Change the foreground color to bright blue
    /// Change the background color to bright blue
    BrightBlue     bright_blue     on_bright_blue,
    /// Change the foreground color to bright magenta
    /// Change the background color to bright magenta
    BrightMagenta  bright_magenta  on_bright_magenta,
    /// Change the foreground color to bright purple
    /// Change the background color to bright purple
    BrightMagenta  bright_purple   on_bright_purple,
    /// Change the foreground color to bright cyan
    /// Change the background color to bright cyan
    BrightCyan     bright_cyan     on_bright_cyan,
    /// Change the foreground color to bright white
    /// Change the background color to bright white
    BrightWhite    bright_white    on_bright_white,
}

#[cfg(test)]
mod tests {
    use crate::OwoColorize;

    #[test]
    fn fg_bg_combo() {
        let test = "test".red().on_blue();
        assert_eq!(test.to_string(), "\x1b[31;44mtest\x1b[0m");
    }

    #[test]
    fn bg_fg_combo() {
        let test = "test".on_blue().red();
        assert_eq!(test.to_string(), "\x1b[31;44mtest\x1b[0m");
    }

    #[test]
    fn fg_overide() {
        let test = "test".green().yellow().red().on_blue();
        assert_eq!(test.to_string(), "\x1b[31;44mtest\x1b[0m");
    }

    #[test]
    fn bg_overide() {
        let test = "test".on_green().on_yellow().on_blue().red();
        assert_eq!(test.to_string(), "\x1b[31;44mtest\x1b[0m");
    }

    #[test]
    fn multiple_overide() {
        let test = "test"
            .on_green()
            .on_yellow()
            .on_red()
            .green()
            .on_blue()
            .red();
        assert_eq!(test.to_string(), "\x1b[31;44mtest\x1b[0m");
    }
}
