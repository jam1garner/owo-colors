use owo_colors::{colors::*, OwoColorize};

fn main() {
    println!("{}", "green".fg::<Green>());
    println!("{}", "yellow".fg::<Yellow>());
    println!("{}", "blue".fg::<Blue>());
    println!("{}", "black".fg::<Black>());
    println!("{}", "red".fg::<Red>());
    println!("{}", "magenta".fg::<Magenta>());
    println!("{}", "white".fg::<White>());
    println!("{}", "cyan".fg::<Cyan>());

    println!("\nBrights\n-------");
    println!("{}", "green".fg::<BrightGreen>());
    println!("{}", "yellow".fg::<BrightYellow>());
    println!("{}", "blue".fg::<BrightBlue>());
    println!("{}", "black".fg::<BrightBlack>());
    println!("{}", "red".fg::<BrightRed>());
    println!("{}", "magenta".fg::<BrightMagenta>());
    println!("{}", "white".fg::<BrightWhite>());
    println!("{}", "cyan".fg::<BrightCyan>());

    println!("\nStyles\n-------");
    println!("{}", "underline".underline());
    println!("{}", "bold".bold());
    println!("{}", "italic".italic());
    println!("{}", "strikethrough".strikethrough());
    println!("{}", "reverse".reversed());
    println!("1{}3", "2".hidden());
    println!("{}", "blink".blink());
    println!("{}", "blink fast".blink_fast());
}
