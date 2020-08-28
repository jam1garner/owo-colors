use owo_colors::{AnsiColors, OwoColorize, Rgb, XtermColors};

fn main() {
    let mut color = AnsiColors::Red;

    println!("{}", "red".dyn_fg(color));

    color = AnsiColors::Blue;

    println!("{}", "blue".dyn_fg(color));

    let color = XtermColors::Fuchsia;

    println!("{}", "fuchsia".dyn_fg(color));

    let color = Rgb(141, 59, 212);

    println!("{}", "custom purple".dyn_fg(color));
}
