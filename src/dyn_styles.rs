use crate::{AnsiColors, Color, DynColor, DynColors};
use core::fmt;

#[cfg(doc)]
use crate::OwoColorize;

/// A runtime-configurable text effect for use with [`Style`]
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum Effect {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    BlinkFast,
    Reversed,
    Hidden,
    Strikethrough,
}

macro_rules! color_methods {
    ($(
        #[$fg_meta:meta] #[$bg_meta:meta] $color:ident $fg_method:ident $bg_method:ident
    ),* $(,)?) => {
        $(
            #[$fg_meta]
            pub fn $fg_method(mut self) -> Self {
                self.fg = Some(DynColors::Ansi(AnsiColors::$color));
                self
            }

            #[$fg_meta]
            pub fn $bg_method(mut self) -> Self {
                self.bg = Some(DynColors::Ansi(AnsiColors::$color));
                self
            }
         )*
    };
}

macro_rules! style_methods {
    ($(#[$meta:meta] $name:ident),* $(,)?) => {
        $(
            #[$meta]
            pub fn $name(mut self) -> Self {
                self.$name = true;
                self
            }
        )*
    };
}

/// A struct with a [`Style`] applied to it
pub struct Styled<T> {
    target: T,
    style: Style,
}

/// A pre-computed style that can be applied to a struct using [`OwoColorize::style`]
#[derive(Debug, Default, Copy, Clone)]
pub struct Style {
    fg: Option<DynColors>,
    bg: Option<DynColors>,
    bold: bool,
    dimmed: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    blink_fast: bool,
    reversed: bool,
    hidden: bool,
    strikethrough: bool,
}

impl Style {
    /// Create a new style to be applied later
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply the style to a given struct to output
    pub fn style<T>(&self, target: T) -> Styled<T> {
        Styled {
            target,
            style: *self,
        }
    }

    /// Set the foreground color generically
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, colors::*};
    ///
    /// println!("{}", "red foreground".fg::<Red>());
    /// ```
    pub fn fg<C: Color>(mut self) -> Self {
        self.fg = Some(C::into_dyncolors());
        self
    }

    /// Set the background color generically.
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, colors::*};
    ///
    /// println!("{}", "black background".bg::<Black>());
    /// ```
    pub fn bg<C: Color>(mut self) -> Self {
        self.bg = Some(C::into_dyncolors());
        self
    }

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

    style_methods! {
        /// Make the text bold
        bold,
        /// Make the text dim
        dimmed,
        /// Make the text italicized
        italic,
        /// Make the text italicized
        underline,
        /// Make the text blink
        blink,
        /// Make the text blink (but fast!)
        blink_fast,
        /// Swap the foreground and background colors
        reversed,
        /// Hide the text
        hidden,
        /// Cross out the text
        strikethrough,
    }

    fn set_effect(&mut self, effect: Effect, to: bool) {
        use Effect::*;
        match effect {
            Bold => self.bold = to,
            Dimmed => self.dimmed = to,
            Italic => self.italic = to,
            Underline => self.underline = to,
            Blink => self.blink = to,
            BlinkFast => self.blink_fast = to,
            Reversed => self.reversed = to,
            Hidden => self.hidden = to,
            Strikethrough => self.strikethrough = to,
        }
    }

    fn set_effects(&mut self, effects: &[Effect], to: bool) {
        for e in effects {
            self.set_effect(*e, to)
        }
    }

    /// Apply a given effect from the style
    pub fn effect(mut self, effect: Effect) -> Self {
        self.set_effect(effect, true);
        self
    }

    /// Remove a given effect from the style
    pub fn remove_effect(mut self, effect: Effect) -> Self {
        self.set_effect(effect, false);
        self
    }

    /// Apply a given set of effects to the style
    pub fn effects(mut self, effects: &[Effect]) -> Self {
        self.set_effects(effects, true);
        self
    }

    /// Remove a given set of effects from the style
    pub fn remove_effects(mut self, effects: &[Effect]) -> Self {
        self.set_effects(effects, false);
        self
    }

    /// Disables all the given effects from the style
    pub fn remove_all_effects(mut self) -> Self {
        self.bold = false;
        self.dimmed = false;
        self.italic = false;
        self.underline = false;
        self.blink = false;
        self.blink_fast = false;
        self.reversed = false;
        self.hidden = false;
        self.strikethrough = false;
        self
    }

    /// Set the foreground color at runtime. Only use if you do not know which color will be used at
    /// compile-time. If the color is constant, use either [`OwoColorize::fg`](crate::OwoColorize::fg) or
    /// a color-specific method, such as [`OwoColorize::green`](crate::OwoColorize::green),
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, AnsiColors};
    ///
    /// println!("{}", "green".color(AnsiColors::Green));
    /// ```
    pub fn color<Color: DynColor>(mut self, color: Color) -> Self {
        self.fg = Some(color.get_dyncolors_fg());
        self
    }

    /// Set the background color at runtime. Only use if you do not know what color to use at
    /// compile-time. If the color is constant, use either [`OwoColorize::bg`](crate::OwoColorize::bg) or
    /// a color-specific method, such as [`OwoColorize::on_yellow`](crate::OwoColorize::on_yellow),
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, AnsiColors};
    ///
    /// println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));
    /// ```
    pub fn on_color<Color: DynColor>(mut self, color: Color) -> Self {
        self.bg = Some(color.get_dyncolors_bg());
        self
    }

    /// Set the foreground color to a specific RGB value.
    pub fn fg_rgb<const R: u8, const G: u8, const B: u8>(mut self) -> Self {
        self.fg = Some(DynColors::Rgb(R, G, B));

        self
    }

    /// Set the background color to a specific RGB value.
    pub fn bg_rgb<const R: u8, const G: u8, const B: u8>(mut self) -> Self {
        self.bg = Some(DynColors::Rgb(R, G, B));

        self
    }

    /// Sets the foreground color to an RGB value.
    pub fn truecolor(mut self, r: u8, g: u8, b: u8) -> Self {
        self.fg = Some(DynColors::Rgb(r, g, b));
        self
    }

    /// Sets the background color to an RGB value.
    pub fn on_truecolor(mut self, r: u8, g: u8, b: u8) -> Self {
        self.bg = Some(DynColors::Rgb(r, g, b));
        self
    }
}

/// Helper to create [`Style`]s more ergonomically
pub fn style() -> Style {
    Style::new()
}

macro_rules! text_effect_fmt {
    ($style:ident, $formatter:ident, $(($attr:ident, $value:literal)),* $(,)?) => {
        $(if $style.$attr {
            $formatter.write_str($value)?;
        })+
    }
}

macro_rules! impl_fmt {
    ($($trait:path),* $(,)?) => {
        $(
            impl<T: $trait> $trait for Styled<T> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

                    let s = &self.style;

                    if let Some(fg) = s.fg {
                        <DynColors as DynColor>::fmt_ansi_fg(&fg, f)?;
                    }

                    if let Some(bg) = s.bg {
                        <DynColors as DynColor>::fmt_ansi_bg(&bg, f)?;
                    }

                    text_effect_fmt!{
                        s, f,
                        (bold,          "\x1b[1m"),
                        (dimmed,        "\x1b[2m"),
                        (italic,        "\x1b[3m"),
                        (underline,     "\x1b[4m"),
                        (blink,         "\x1b[5m"),
                        (blink_fast,    "\x1b[6m"),
                        (reversed,      "\x1b[7m"),
                        (hidden,        "\x1b[8m"),
                        (strikethrough, "\x1b[9m"),
                    }

                    <T as $trait>::fmt(&self.target, f)?;

                    if s.fg.is_some()
                        || s.bg.is_some()
                        || s.bold
                        || s.dimmed
                        || s.italic
                        || s.underline
                        || s.blink
                        || s.blink_fast
                        || s.reversed
                        || s.hidden
                        || s.strikethrough
                    {
                        f.write_str("\x1b[0m")?;
                    }

                    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnsiColors, OwoColorize};

    #[test]
    fn test_it() {
        let style = Style::new()
            .bright_white()
            .on_blue()
            .bold()
            .dimmed()
            .italic()
            .underline()
            .blink()
            //.blink_fast()
            //.reversed()
            //.hidden()
            .strikethrough();
        let s = style.style("TEST");
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "\u{1b}[97m\u{1b}[44m\u{1b}[1m\u{1b}[2m\u{1b}[3m\u{1b}[4m\u{1b}[5m\u{1b}[9mTEST\u{1b}[0m");
    }

    #[test]
    fn test_effects() {
        use Effect::*;
        let style = Style::new().effects(&[Strikethrough, Underline]);

        let s = style.style("TEST");
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "\u{1b}[4m\u{1b}[9mTEST\u{1b}[0m");
    }

    #[test]
    fn test_color() {
        let style = Style::new()
            .color(AnsiColors::White)
            .on_color(AnsiColors::Black);

        let s = style.style("TEST");
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "\u{1b}[37m\u{1b}[40mTEST\u{1b}[0m");
    }

    #[test]
    fn test_truecolor() {
        let style = Style::new().truecolor(255, 255, 255).on_truecolor(0, 0, 0);

        let s = style.style("TEST");
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(
            &s2,
            "\u{1b}[38;2;255;255;255m\u{1b}[48;2;0;0;0mTEST\u{1b}[0m"
        );
    }

    #[test]
    fn test_string_reference() {
        let style = Style::new().truecolor(255, 255, 255).on_truecolor(0, 0, 0);

        let string = String::from("TEST");
        let s = style.style(&string);
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(
            &s2,
            "\u{1b}[38;2;255;255;255m\u{1b}[48;2;0;0;0mTEST\u{1b}[0m"
        );
    }

    #[test]
    fn test_owocolorize() {
        let style = Style::new().bright_white().on_blue();

        let s = "TEST".style(style);
        let s2 = format!("{}", &s);
        println!("{}", &s2);
        assert_eq!(&s2, "\u{1b}[97m\u{1b}[44mTEST\u{1b}[0m");
    }
}
