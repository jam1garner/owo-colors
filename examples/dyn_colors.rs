use owo_colors::{AnsiColors, OwoColorize, Rgb, XtermColors};

fn main() {
    let mut color = AnsiColors::Red;
    println!("{}", "red".color(color));

    color = AnsiColors::Blue;
    println!("{}", "blue".color(color));

    let color = XtermColors::Fuchsia;
    println!("{}", "fuchsia".color(color));

    let color = Rgb(141, 59, 212);
    println!("{}", "custom purple".color(color));
}
