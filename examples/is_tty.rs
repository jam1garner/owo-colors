use owo_colors::OwoColorize;

fn main() {
    println!("{}", "This will be red if viewed through a tty!".if_stdout_tty(|x| x.red()));
}
