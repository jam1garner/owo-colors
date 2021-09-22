use owo_colors::{OwoColorize, Stream::Stdout};

fn main() {
    println!(
        "{}",
        "This will be red if viewed through a tty!".if_supports_color(Stdout, |x| x.red())
    );
}
