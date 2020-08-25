use super::colors::*;
use super::OwoColorize;

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
