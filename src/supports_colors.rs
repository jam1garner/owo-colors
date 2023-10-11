use core::fmt;

mod private {
    pub(super) trait Sealed {}
}

/// A possible stream source.
///
/// This can be used
#[derive(Clone, Copy, Debug)]
pub enum OutputStream {
    /// Standard output.
    Stdout,

    /// Standard error.
    Stderr,

    /// Standard input. Only used to retain compatibility with supports-colors v1.
    #[doc(hidden)]
    #[deprecated(
        since = "3.7.0",
        note = "This is only used to retain compatibility with supports-colors v1."
    )]
    Stdin,
}

#[cfg(feature = "supports-colors")]
impl From<supports_color::Stream> for OutputStream {
    fn from(stream: supports_color::Stream) -> Self {
        match stream {
            supports_color::Stream::Stdout => OutputStream::Stdout,
            supports_color::Stream::Stderr => OutputStream::Stderr,
            #[allow(deprecated)]
            supports_color::Stream::Stdin => OutputStream::Stdin,
        }
    }
}

#[cfg(feature = "supports-colors-2")]
impl From<supports_color_2::Stream> for OutputStream {
    fn from(stream: supports_color_2::Stream) -> Self {
        match stream {
            supports_color_2::Stream::Stdout => OutputStream::Stdout,
            supports_color_2::Stream::Stderr => OutputStream::Stderr,
        }
    }
}

#[cfg(any(feature = "supports-colors", feature = "supports-colors-2"))]
/// A display wrapper which applies a transformation based on if the given stream supports
/// colored terminal output
pub struct SupportsColorsDisplay<'a, InVal, Out, ApplyFn>(
    pub(crate) &'a InVal,
    pub(crate) ApplyFn,
    pub(crate) OutputStream,
)
where
    InVal: ?Sized,
    ApplyFn: Fn(&'a InVal) -> Out;

use crate::OVERRIDE;

macro_rules! impl_fmt_for {
    ($($trait:path),* $(,)?) => {
        $(
            impl<'a, In, Out, F> $trait for SupportsColorsDisplay<'a, In, Out, F>
                where In: $trait,
                      Out: $trait,
                      F: Fn(&'a In) -> Out,
            {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let (force_enabled, force_disabled) = OVERRIDE.is_force_enabled_or_disabled();
                    if force_enabled || (on_cached(self.2) && !force_disabled) {
                        <Out as $trait>::fmt(&self.1(self.0), f)
                    } else {
                        <In as $trait>::fmt(self.0, f)
                    }
                }
            }
        )*
    };
}

/// Use supports-colors v2 if it is enabled.
#[cfg(feature = "supports-colors-2")]
fn on_cached(stream: OutputStream) -> bool {
    let stream = match stream {
        OutputStream::Stdout => supports_color_2::Stream::Stdout,
        OutputStream::Stderr => supports_color_2::Stream::Stderr,
        #[allow(deprecated)]
        OutputStream::Stdin => {
            panic!("stdin is not supported if supports-colors-2 is enabled")
        }
    };
    supports_color_2::on_cached(stream)
        .map(|level| level.has_basic)
        .unwrap_or(false)
}

/// Use supports-colors v1 if v2 is not enabled.
#[cfg(all(feature = "supports-color", not(feature = "supports-colors-2")))]
fn on_cached(stream: OutputStream) -> bool {
    let stream = match stream {
        OutputStream::Stdout => supports_color::Stream::Stdout,
        OutputStream::Stderr => supports_color::Stream::Stderr,
        #[allow(deprecated)]
        OutputStream::Stdin => supports_color::Stream::Stdin,
    };
    supports_color::on_cached(stream)
        .map(|level| level.has_basic)
        .unwrap_or(false)
}

impl_fmt_for! {
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
