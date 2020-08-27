use owo_colors::{colors::xterm, OwoColorize};

fn main() {
    println!("{}", "Electric violet".fg::<xterm::ElectricViolet>());
    println!("{}", "Matrix".fg::<xterm::Matrix>());
    println!("{}", "Flirt".fg::<xterm::Flirt>());
    println!("{}", "Cyan2".fg::<xterm::Cyan2>());
    println!("{}", "Cyan".fg::<xterm::Cyan>());
    println!("{}", "Lime".fg::<xterm::Lime>());
    println!("{}", "Jade".fg::<xterm::Jade>());
    println!("{}", "Reef".fg::<xterm::Reef>());
}
