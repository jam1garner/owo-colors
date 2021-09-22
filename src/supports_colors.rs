use core::fmt;

/// A display which applies a transformation based on if the given stream is a tty
pub struct SupportsColorsDisplay<'a, InVal, Out, ApplyFn>(
    pub(crate) &'a InVal,
    pub(crate) ApplyFn,
    pub(crate) supports_color::Stream,
)
where
    InVal: ?Sized,
    ApplyFn: Fn(&'a InVal) -> Out;

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
                    if supports_color::on_cached(self.2).map(|level| level.has_basic).unwrap_or(false) {
                        <Out as $trait>::fmt(&self.1(self.0), f)
                    } else {
                        <In as $trait>::fmt(self.0, f)
                    }
                }
            }
        )*
    };
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
