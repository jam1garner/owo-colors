use core::fmt;
use core::fmt::{Display, Write};

use super::colors::*;
use super::OwoColorize;
use crate::colors::css::Lavender;
use crate::{AnsiColors, DynColors};

struct BufferWriter<'a> {
    pointer: &'a mut [u8],
    offset: usize,
}

impl Write for BufferWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // only used during tests, therefore we can just assert
        assert!(self.pointer.len() - self.offset >= s.len());

        let bytes = s.as_bytes();
        self.pointer[self.offset..self.offset + bytes.len()].copy_from_slice(bytes);
        self.offset += bytes.len();

        Ok(())
    }
}

pub(crate) fn assert_str<T: Display>(lhs: T, rhs: &str) {
    // allocate enough in our standard buffer for all examples
    let mut buffer = [0u8; 256];
    let mut writer = BufferWriter {
        pointer: &mut buffer,
        offset: 0,
    };

    write!(&mut writer, "{}", lhs).expect("able to write to buffer");
    let length = writer.offset;

    let lhs = core::str::from_utf8(&buffer[..length]).expect("should be valid utf8");

    assert_eq!(lhs, rhs);
}

#[test]
fn test_assert_str() {
    assert_str("TEST", "TEST");
}

#[test]
#[should_panic]
fn test_assert_str_neq() {
    assert_str("TEST123", "TEST");
}

#[test]
fn test_fg() {
    assert_str("test".fg::<Black>(), "\x1b[30mtest\x1b[39m");
    assert_str("blah blah".red(), "\x1b[31mblah blah\x1b[39m");
}

#[test]
fn test_bg() {
    assert_str("test".bg::<Black>(), "\x1b[40mtest\x1b[49m");
    assert_str("blah blah".on_red(), "\x1b[41mblah blah\x1b[49m");
}

#[test]
fn test_dyn_fg() {
    assert_str("test".color(AnsiColors::Black), "\x1b[30mtest\x1b[39m");
    assert_str(
        "blah blah".color(AnsiColors::Red),
        "\x1b[31mblah blah\x1b[39m",
    );
}

#[test]
fn test_dyn_bg() {
    assert_str("test".on_color(AnsiColors::Black), "\x1b[40mtest\x1b[49m");
    assert_str(
        "blah blah".on_color(AnsiColors::Red),
        "\x1b[41mblah blah\x1b[49m",
    );
}

#[test]
fn test_hex() {
    assert_str(
        format_args!("{:08X}", 0xa.red()),
        "\x1b[31m0000000A\x1b[39m",
    );
}

#[test]
fn test_css_name() {
    assert_str(
        "test".fg::<Lavender>(),
        "\x1b[38;2;230;230;250mtest\x1b[39m",
    );
}

#[test]
fn test_parse() {
    macro_rules! assert_parse {
        ($($str:literal == $eq:expr),* $(,)?) => {
            $(
                assert_eq!($eq, $str.parse().unwrap());
             )*
        }
    }

    assert_parse!(
        "yellow" == DynColors::Ansi(AnsiColors::Yellow),
        "blue" == DynColors::Ansi(AnsiColors::Blue),
        "#eb4034" == DynColors::Rgb(235, 64, 52),
    );
}

#[test]
fn default_color() {
    assert_str(
        format_args!("red red red {} no color", "default color".default_color()).red(),
        "\x1b[31mred red red \x1b[39mdefault color\x1b[39m no color\x1b[39m",
    );
}
