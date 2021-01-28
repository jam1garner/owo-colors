//! This crate provides [`OwoColorize`](OwoColorize), an extension trait for colorizing a
//! formatter.
//!
//! ## Example
//!
//! ```rust
//! use owo_colors::OwoColorize;
//!
//! fn main() {
//!     // Foreground colors
//!     println!("My number is {:#x}!", 10.green());
//!     // Background colors
//!     println!("My number is not {}!", 4.on_red());
//! }
//! ```
//!
//! ## Generically color
//!
//! ```rust
//! use owo_colors::OwoColorize;
//! use owo_colors::colors::*;
//!
//! fn main() {
//!     // Generically color
//!     println!("My number might be {}!", 4.fg::<Black>().bg::<Yellow>());
//! }
//! ```
//!
//! ## Stylize
//!
//! ```rust
//! use owo_colors::OwoColorize;
//!
//! println!("{}", "strikethrough".strikethrough());
//! ```
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "custom", feature(min_const_generics))]
#![doc(html_logo_url = "https://jam1.re/img/rust_owo.svg")]

use core::fmt;
use core::marker::PhantomData;

/// A trait for describing a type which can be used with [`FgColorDisplay`](FgColorDisplay) or
/// [`BgCBgColorDisplay`](BgColorDisplay)
pub trait Color {
    const ANSI_FG: &'static str;
    const ANSI_BG: &'static str;

    #[doc(hidden)]
    fn into_dyncolors() -> crate::DynColors;
}

/// A trait describing a runtime-configurable color which can displayed using [`FgDynColorDisplay`](FgDynColorDisplay)
/// or [`BgDynColorDisplay`](BgDynColorDisplay). If your color will be known at compile time it
/// is recommended you avoid this.
pub trait DynColor {
    fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    #[doc(hidden)]
    fn get_dyncolors_fg(&self) -> DynColors;
    #[doc(hidden)]
    fn get_dyncolors_bg(&self) -> DynColors;
}

/// Transparent wrapper around a type which implements all the formatters the wrapped type does,
/// with the addition of changing the foreground color. Recommended to be constructed using
/// [`OwoColorize`](OwoColorize).
#[repr(transparent)]
pub struct FgColorDisplay<'a, C: Color, T>(&'a T, PhantomData<C>);

/// Transparent wrapper around a type which implements all the formatters the wrapped type does,
/// with the addition of changing the background color. Recommended to be constructed using
/// [`OwoColorize`](OwoColorize).
#[repr(transparent)]
pub struct BgColorDisplay<'a, C: Color, T>(&'a T, PhantomData<C>);

/// Wrapper around a type which implements all the formatters the wrapped type does,
/// with the addition of changing the foreground color. Is not recommended unless compile-time
/// coloring is not an option.
pub struct FgDynColorDisplay<'a, Color: DynColor, T>(&'a T, Color);

/// Wrapper around a type which implements all the formatters the wrapped type does,
/// with the addition of changing the background color. Is not recommended unless compile-time
/// coloring is not an option.
pub struct BgDynColorDisplay<'a, Color: DynColor, T>(&'a T, Color);

macro_rules! color_methods {
    ($(
        #[$fg_meta:meta] #[$bg_meta:meta] $color:ident $fg_method:ident $bg_method:ident
    ),* $(,)?) => {
        $(
            #[$fg_meta]
            #[inline(always)]
            fn $fg_method<'a>(&'a self) -> FgColorDisplay<'a, colors::$color, Self> {
                FgColorDisplay(self, PhantomData)
            }

            #[$fg_meta]
            #[inline(always)]
            fn $bg_method<'a>(&'a self) -> BgColorDisplay<'a, colors::$color, Self> {
                BgColorDisplay(self, PhantomData)
            }
         )*
    };
}

macro_rules! style_methods {
    ($(#[$meta:meta] $name:ident $ty:ident),* $(,)?) => {
        $(
            #[$meta]
            #[inline(always)]
            fn $name<'a>(&'a self) -> styles::$ty<'a, Self> {
                styles::$ty(self)
            }
         )*
    };
}

/// Extension trait for colorizing a type which implements any std formatter
/// ([`Display`](core::fmt::Display), [`Debug`](core::fmt::Debug), [`UpperHex`](core::fmt::UpperHex),
/// etc.)
///
/// ## Example
///
/// ```rust
/// use owo_colors::OwoColorize;
///
/// fn main() {
///     println!("My number is {:#x}!", 10.green());
///     println!("My number is not {}!", 4.on_red());
/// }
/// ```
///
/// ## How to decide which method to use
///
/// **Do you have a specific color you want to use?**
///
/// Use the specific color's method, such as [`blue`](OwoColorize::blue) or
/// [`on_green`](OwoColorize::on_green).
///
///
/// **Do you want your colors configurable via generics?**
///
/// Use [`fg`](OwoColorize::fg) and [`bg`](OwoColorize::bg) to make it compile-time configurable.
///
///
/// **Do you need to pick a color at runtime?**
///
/// Use the [`color`](OwoColorize::color), [`on_color`](OwoColorize::on_color),
/// [`truecolor`](OwoColorize::truecolor) or [`on_truecolor`](OwoColorize::on_truecolor).
///
/// **Do you need some other text modifier?**
///
/// * [`bold`](OwoColorize::bold)
/// * [`dimmed`](OwoColorize::dimmed)
/// * [`italic`](OwoColorize::italic)
/// * [`underline`](OwoColorize::underline)
/// * [`blink`](OwoColorize::blink)
/// * [`blink_fast`](OwoColorize::blink_fast)
/// * [`reversed`](OwoColorize::reversed)
/// * [`hidden`](OwoColorize::hidden)
/// * [`strikethrough`](OwoColorize::strikethrough)
///
/// **Do you want it to only display colors if it's a terminal?**
///
/// 1. Enable the `tty` feature
/// 2. Colorize inside [`if_stdout_tty`](OwoColorize::if_stdout_tty) or
/// [`if_stdout_tty`](OwoColorize::if_stderr_tty)
///
pub trait OwoColorize: Sized {
    /// Set the foreground color generically
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, colors::*};
    ///
    /// println!("{}", "red foreground".fg::<Red>());
    /// ```
    #[inline(always)]
    fn fg<'a, C: Color>(&'a self) -> FgColorDisplay<'a, C, Self> {
        FgColorDisplay(self, PhantomData)
    }

    /// Set the background color generically.
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, colors::*};
    ///
    /// println!("{}", "black background".bg::<Black>());
    /// ```
    #[inline(always)]
    fn bg<'a, C: Color>(&'a self) -> BgColorDisplay<'a, C, Self> {
        BgColorDisplay(self, PhantomData)
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
        bold BoldDisplay,
        /// Make the text dim
        dimmed DimDisplay,
        /// Make the text italicized
        italic ItalicDisplay,
        /// Make the text italicized
        underline UnderlineDisplay,
        /// Make the text blink
        blink BlinkDisplay,
        /// Make the text blink (but fast!)
        blink_fast BlinkFastDisplay,
        /// Swap the foreground and background colors
        reversed ReversedDisplay,
        /// Hide the text
        hidden HiddenDisplay,
        /// Cross out the text
        strikethrough StrikeThroughDisplay,
    }

    /// Set the foreground color at runtime. Only use if you do not know which color will be used at
    /// compile-time. If the color is constant, use either [`OwoColorize::fg`](OwoColorize::fg) or
    /// a color-specific method, such as [`OwoColorize::green`](OwoColorize::green),
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, AnsiColors};
    ///
    /// fn main() {
    ///     println!("{}", "green".color(AnsiColors::Green));
    /// }
    /// ```
    #[inline(always)]
    fn color<'a, Color: DynColor>(&'a self, color: Color) -> FgDynColorDisplay<'a, Color, Self> {
        FgDynColorDisplay(self, color)
    }

    /// Set the background color at runtime. Only use if you do not know what color to use at
    /// compile-time. If the color is constant, use either [`OwoColorize::bg`](OwoColorize::bg) or
    /// a color-specific method, such as [`OwoColorize::on_yellow`](OwoColorize::on_yellow),
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, AnsiColors};
    ///
    /// fn main() {
    ///     println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));
    /// }
    /// ```
    #[inline(always)]
    fn on_color<'a, Color: DynColor>(&'a self, color: Color) -> BgDynColorDisplay<'a, Color, Self> {
        BgDynColorDisplay(self, color)
    }

    /// Set the foreground color to a specific RGB value.
    ///
    /// **Requires**: nightly and the `custom` feature.
    ///
    /// If nightly is not preferable for you, use [`OwoColorize::truecolor`](OwoColorize::truecolor)
    #[cfg(feature = "custom")]
    fn fg_rgb<'a, const R: u8, const G: u8, const B: u8>(
        &'a self,
    ) -> FgColorDisplay<'a, colors::CustomColor<R, G, B>, Self> {
        FgColorDisplay(self, PhantomData)
    }

    /// Set the background color to a specific RGB value.
    ///
    /// **Requires**: nightly and the `custom` feature.
    ///
    /// If nightly is not preferable for you, use [`OwoColorize::on_truecolor`](OwoColorize::on_truecolor)
    #[cfg(feature = "custom")]
    fn bg_rgb<'a, const R: u8, const G: u8, const B: u8>(
        &'a self,
    ) -> BgColorDisplay<'a, colors::CustomColor<R, G, B>, Self> {
        BgColorDisplay(self, PhantomData)
    }

    /// Sets the foreground color to an RGB value.
    #[inline(always)]
    fn truecolor<'a>(&'a self, r: u8, g: u8, b: u8) -> FgDynColorDisplay<'a, Rgb, Self> {
        FgDynColorDisplay(self, Rgb(r, g, b))
    }

    /// Sets the background color to an RGB value.
    #[inline(always)]
    fn on_truecolor<'a>(&'a self, r: u8, g: u8, b: u8) -> BgDynColorDisplay<'a, Rgb, Self> {
        BgDynColorDisplay(self, Rgb(r, g, b))
    }

    /// Apply a runtime-determined style
    fn style(&self, style: Style) -> Styled<&Self> {
        style.style(self)
    }
    
    /// Apply a given transformation function to all formatters if stdout is a tty console
    /// allowing you to conditionally apply given styles/colors.
    ///
    /// Requires the `tty` feature.
    ///
    /// ```rust
    /// use owo_colors::{OwoColorize, Style};
    ///
    /// fn main() {
    ///     println!(
    ///         "{}",
    ///         "bright cyan if this is terminal output"
    ///             .if_stdout_tty(|text| text.bright_cyan())
    ///     );
    ///
    ///     // applying multiple at both
    ///     println!(
    ///         "{}",
    ///         "bright cyan AND underlined(?!) if this is terminal output"
    ///             .if_stdout_tty(|text| text.style(
    ///                 Style::new()
    ///                     .bright_cyan()
    ///                     .underline()
    ///             ))
    ///     );
    /// }
    /// ```
    #[cfg(feature = "tty")]
    fn if_stdout_tty<'a, Out, ApplyFn>(
        &'a self,
        apply: ApplyFn
    ) -> TtyDisplay<'a, StdOut, Self, Out, ApplyFn>
        where ApplyFn: Fn(&'a Self) -> Out
    {
        TtyDisplay(self, apply, StdOut)
    }
    
    /// Apply a given transformation function to all formatters if stderr is a tty console
    /// allowing you to conditionally apply given styles/colors.
    ///
    /// Requires the `tty` feature.
    ///
    /// ```rust
    /// use owo_colors::OwoColorize;
    ///
    /// fn main() {
    ///     eprintln!(
    ///         "{}",
    ///         "woah! error! if this is terminal output, it's red"
    ///             .if_stderr_tty(|text| text.bright_red())
    ///     );
    /// }
    /// ```
    #[cfg(feature = "tty")]
    fn if_stderr_tty<'a, Out, ApplyFn>(
        &'a self,
        apply: ApplyFn
    ) -> TtyDisplay<'a, StdErr, Self, Out, ApplyFn>
        where ApplyFn: Fn(&'a Self) -> Out
    {
        TtyDisplay(self, apply, StdErr)
    }
}

pub use colors::{ansi_colors::AnsiColors, dynamic::Rgb, xterm::dynamic::XtermColors};

// TODO: figure out some wait to only implement for fmt::Display | fmt::Debug | ...
impl<D: Sized> OwoColorize for D {}

mod dyn_colors;
pub use dyn_colors::*;

mod dyn_styles;
pub use dyn_styles::*;

#[cfg(feature = "tty")]
mod tty_display;

#[cfg(feature = "tty")]
pub use tty_display::TtyDisplay;

#[cfg(feature = "tty")]
use tty_display::{StdOut, StdErr};

/// Color types for used for being generic over the color
pub mod colors;

/// Different display styles (strikethrough, bold, etc.)
pub mod styles;

/// Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from
/// [`colored`](https://docs.rs/colored) to owo-colors.
///
/// Just replace:
///
/// ```rust
/// # mod colored {}
/// use colored::*;
/// ```
///
/// with
///
/// ```rust
/// use owo_colors::colored::*;
/// ```
pub mod colored {
    pub use crate::AnsiColors as Color;
    pub use crate::OwoColorize;
}

#[cfg(test)]
mod tests;
