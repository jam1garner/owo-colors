use core::fmt;

pub struct StdOut;
pub struct StdErr;

/// Trait implemented to provide fake const generics for plugging in given streams
pub trait IsTty {
    const TTY: atty::Stream;
}

impl IsTty for StdOut {
    const TTY: atty::Stream = atty::Stream::Stdout;
}

impl IsTty for StdErr {
    const TTY: atty::Stream = atty::Stream::Stderr;
}

/// A display which applies a transformation based on if the given stream is a tty
pub struct TtyDisplay<'a, Tty, InVal, Out, ApplyFn>(
    pub(crate) &'a InVal,
    pub(crate) ApplyFn,
    pub(crate) Tty,
)
where
    Tty: IsTty,
    InVal: ?Sized,
    ApplyFn: Fn(&'a InVal) -> Out;

macro_rules! impl_fmt_for {
    ($($trait:path),* $(,)?) => {
        $(
            impl<'a, Tty: IsTty, In: $trait, Out: $trait, F: Fn(&'a In) -> Out> $trait for TtyDisplay<'a, Tty, In, Out, F> {
                #[inline(always)]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    if atty::is(Tty::TTY) {
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
