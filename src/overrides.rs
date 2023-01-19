use core::sync::atomic::{AtomicU8, Ordering};

/// possible stream sources
#[derive(Debug, Copy, Clone)]
pub enum Stream {
    Stdout,
    Stderr,
}

#[cfg(feature = "supports-colors")]
impl From<supports_color::Stream> for Stream {
    fn from(value: supports_color::Stream) -> Self {
        match value {
            supports_color::Stream::Stdout => Self::Stdout,
            supports_color::Stream::Stderr => Self::Stderr,
        }
    }
}

/// Set an override value for whether or not colors are supported using
/// [`set_override`] while executing the closure provided.
///
/// Once the function has executed the value will be reset to the previous set
/// (or unset) override.
///
/// This is especially useful in use-cases where one would like to temporarily
/// override the supported color set, without impacting previous configurations.
///
/// ```
/// # use owo_colors::Stream;
/// # use owo_colors::{OwoColorize, set_override, unset_override, with_override};
/// # use owo_colors::colors::Black;
/// #
/// set_override(false);
/// assert_eq!("example".if_supports_color(Stream::Stdout, |value| value.bg::<Black>()).to_string(), "example");
///
/// with_override(true, || {
///     assert_eq!("example".if_supports_color(Stream::Stdout, |value| value.bg::<Black>()).to_string(), "\x1b[40mexample\x1b[49m");
/// });
///
/// assert_eq!("example".if_supports_color(Stream::Stdout, |value| value.bg::<Black>()).to_string(), "example");
/// # unset_override() // make sure that other doc tests are not impacted
/// ```
#[cfg(feature = "override")]
pub fn with_override<T, F: FnOnce() -> T>(value: Override, f: F) -> T {
    let previous = OVERRIDE.load();
    OVERRIDE.store(value);

    let value = f();

    OVERRIDE.store(previous);

    value
}

/// Set an override value for whether or not colors are supported.
///
/// If `true` is passed,
/// [`if_supports_color`](crate::OwoColorize::if_supports_color) will always act
/// as if colors are supported.
///
/// If `false` is passed,
/// [`if_supports_color`](crate::OwoColorize::if_supports_color) will always act
/// as if colors are **not** supported.
///
/// This behavior can be disabled using [`unset_override`], allowing
/// `owo-colors` to return to inferring if colors are supported.
#[cfg(feature = "override")]
#[deprecated(
    since = "4.0.0",
    note = "use `override_ansi(ColorOverride::Enable)` or `override_ansi(ColorOverride::Disable)` instead"
)]
pub fn set_override(enabled: bool) {
    OVERRIDE.store_ansi(if enabled {
        ColorOverride::Enable
    } else {
        ColorOverride::Disable
    });
}

/// Remove any override value for whether or not colors are supported. This
/// means [`if_supports_color`](crate::OwoColorize::if_supports_color) will
/// resume checking if the given terminal output ([`Stream`](crate::Stream))
/// supports colors.
///
/// This override can be set using [`set_override`].
#[cfg(feature = "override")]
#[deprecated(
    since = "4.0.0",
    note = "use `override_ansi(ColorOverride::None)` instead"
)]
pub fn unset_override() {
    OVERRIDE.store_ansi(ColorOverride::None);
}

pub fn override_ansi(value: ColorOverride) {
    OVERRIDE.store_ansi(value)
}

pub fn override_xterm(value: ColorOverride) {
    OVERRIDE.store_xterm(value)
}

pub fn override_truecolor(value: ColorOverride) {
    OVERRIDE.store_xterm(value)
}

pub fn override_set(value: Override) {
    OVERRIDE.store(value)
}

pub fn override_status() -> Override {
    OVERRIDE.load()
}

#[cfg(not(feature = "supports-colors"))]
pub fn supports(stream: Stream) -> ColorLevel {
    OVERRIDE.load().to_level(false)
}

#[cfg(feature = "supports-colors")]
pub fn supports(stream: Stream) -> ColorLevel {
    let default = supports_color::on_cached(stream.into())
        .map(Into::into)
        .unwrap_or_else(ColorLevel::none);

    let level = OVERRIDE.load();

    ColorLevel {
        ansi: level.ansi.to_bool(default.ansi),
        xterm: level.xterm.to_bool(default.xterm),
        truecolor: level.truecolor.to_bool(default.truecolor),
    }
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct ColorLevel {
    ansi: bool,
    xterm: bool,
    truecolor: bool,
}

impl ColorLevel {
    pub const fn none() -> Self {
        Self {
            ansi: false,
            xterm: false,
            truecolor: false,
        }
    }

    pub const fn ansi(&self) -> bool {
        self.ansi
    }

    pub const fn xterm(&self) -> bool {
        self.xterm
    }

    pub const fn truecolor(&self) -> bool {
        self.truecolor
    }
}

impl From<supports_color::ColorLevel> for ColorLevel {
    fn from(value: supports_color::ColorLevel) -> Self {
        Self {
            ansi: value.has_basic,
            xterm: value.has_256,
            truecolor: value.has_16m,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum ColorOverride {
    None,
    Enable,
    Disable,
}

impl ColorOverride {
    const fn to_num(self) -> u8 {
        match self {
            Self::None => 0b00,
            Self::Enable => 0b01,
            Self::Disable => 0b10,
        }
    }

    const fn to_bool(self, default: bool) -> bool {
        match self {
            Self::None => default,
            Self::Enable => true,
            Self::Disable => false,
        }
    }

    const fn from_num(value: u8) -> Self {
        match value {
            0b01 => Self::Enable,
            0b10 => Self::Disable,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Override {
    /// Support for basic ANSI escape codes
    ///
    /// 16 color codes with bold/italic and background
    ansi: ColorOverride,

    /// Support for 256 colors
    ///
    /// 256 color palette: 216 colors + 16 ANSI + 24 gray (colors are 24-bit)
    xterm: ColorOverride,

    /// Support for 16 million colors
    ///
    /// 24-bit truecolor: "888" colors (aka 16 million)
    truecolor: ColorOverride,
}

impl Override {
    pub const fn enable() -> Self {
        Self {
            ansi: ColorOverride::Enable,
            xterm: ColorOverride::Enable,
            truecolor: ColorOverride::Enable,
        }
    }

    pub const fn disable() -> Self {
        Self {
            ansi: ColorOverride::Disable,
            xterm: ColorOverride::Disable,
            truecolor: ColorOverride::Disable,
        }
    }

    pub const fn none() -> Self {
        Self {
            ansi: ColorOverride::None,
            xterm: ColorOverride::None,
            truecolor: ColorOverride::None,
        }
    }

    pub const fn with_ansi(mut self, value: ColorOverride) -> Self {
        self.ansi = value;
        self
    }

    pub const fn with_xterm(mut self, value: ColorOverride) -> Self {
        self.xterm = value;
        self
    }

    pub const fn with_truecolor(mut self, value: ColorOverride) -> Self {
        self.truecolor = value;
        self
    }

    const fn to_num(self) -> u8 {
        self.truecolor.to_num() | (self.xterm.to_num() << 2) | (self.ansi.to_num() << 4)
    }

    const fn from_num(value: u8) -> Self {
        let truecolor = ColorOverride::from_num(value & 0b11);
        let xterm = ColorOverride::from_num((value >> 2) & 0b11);
        let ansi = ColorOverride::from_num((value >> 4) & 0b11);

        Self {
            truecolor,
            xterm,
            ansi,
        }
    }

    const fn to_level(self, default: bool) -> ColorLevel {
        ColorLevel {
            ansi: self.ansi.to_bool(default),
            xterm: self.xterm.to_bool(default),
            truecolor: self.truecolor.to_bool(default),
        }
    }
}

impl Default for Override {
    fn default() -> Self {
        Self::none()
    }
}

pub(crate) static OVERRIDE: AtomicOverride = AtomicOverride::none();

/// Stores global [`Override`], the type layout is:
///
/// `__ AA XX TT`
///
/// The first two bits are unused, while `AA` is for ansi, `XX` for xterm, `TT`
/// for truecolor
pub(crate) struct AtomicOverride(AtomicU8);

impl AtomicOverride {
    const fn none() -> Self {
        Self(AtomicU8::new(Override::none().to_num()))
    }

    fn load(&self) -> Override {
        let value = self.0.load(Ordering::SeqCst);

        Override::from_num(value)
    }

    fn store(&self, value: Override) {
        let value = value.to_num();

        self.0.store(value, Ordering::SeqCst);
    }

    fn store_ansi(&self, value: ColorOverride) {
        let mut base = self.load();
        base.ansi = value;

        self.store(base);
    }

    fn store_xterm(&self, value: ColorOverride) {
        let mut base = self.load();
        base.xterm = value;

        self.store(base);
    }

    fn store_truecolor(&self, value: ColorOverride) {
        let mut base = self.load();
        base.truecolor = value;

        self.store(base);
    }
}
