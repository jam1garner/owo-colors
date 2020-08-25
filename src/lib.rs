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
#![doc(html_logo_url = "https://jam1.re/img/rust_owo.svg")]
use core::marker::PhantomData;

/// A trait for describing a type which can be used with [`FgColorDisplay`](FgColorDisplay) or
/// [`BgCBgColorDisplay`](BgColorDisplay)
pub trait Color {
    const ANSI_FG: &'static str;
    const ANSI_BG: &'static str;
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

macro_rules! color_methods {
    ($(
        $color:ident $fg_method:ident $bg_method:ident
    ),* $(,)?) => {
        $(
            /// Change the foreground color
            #[inline(always)]
            fn $fg_method<'a>(&'a self) -> FgColorDisplay<'a, colors::$color, Self> {
                FgColorDisplay(self, PhantomData)
            }
            
            /// Change the background color
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

/// Extension trait for colorizing a type which implements a formatter
/// ([`Display`](core::fmt::Display), [`Debug`](core::fmt::Debug), etc.)
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
pub trait OwoColorize: Sized {
    /// Set the foreground color generically
    #[inline(always)]
    fn fg<'a, C: Color>(&'a self) -> FgColorDisplay<'a, C, Self> {
        FgColorDisplay(self, PhantomData)
    }
 
    /// Set the background color generically
    #[inline(always)]
    fn bg<'a, C: Color>(&'a self) -> BgColorDisplay<'a, C, Self> {
        BgColorDisplay(self, PhantomData)
    }

    style_methods!{
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

    color_methods!{
        Black    black    on_black,
        Red      red      on_red,
        Green    green    on_green,
        Yellow   yellow   on_yellow,
        Blue     blue     on_blue,
        Magenta  magenta  on_magenta,
        Magenta  purple   on_purple,
        Cyan     cyan     on_cyan,
        White    white    on_white,

        BrightBlack    bright_black    on_bright_black,
        BrightRed      bright_red      on_bright_red,
        BrightGreen    bright_green    on_bright_green,
        BrightYellow   bright_yellow   on_bright_yellow,
        BrightBlue     bright_blue     on_bright_blue,
        BrightMagenta  bright_magenta  on_bright_magenta,
        BrightMagenta  bright_purple   on_bright_purple,
        BrightCyan     bright_cyan     on_bright_cyan,
        BrightWhite    bright_white    on_bright_white,
    }
}

// TODO: figure out some wait to only implement for fmt::Display | fmt::Debug | ...
impl<D: Sized> OwoColorize for D {}


/// Color types for used for being generic over the color
pub mod colors;
/// Different display styles (strikethrough, bold, etc.)
pub mod styles;

#[cfg(test)]
mod tests;
