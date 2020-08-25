# <img src="https://jam1.re/img/rust_owo.svg" height="100"> Colors

A zero-allocation no_std-compatible zero-cost way to add color to your Rust terminal.

**Supports:**

* [x] [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
* [x] [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
* [x] [Octal](https://doc.rust-lang.org/std/fmt/trait.Octal.html)
* [x] [LowerHex](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html)
* [x] [UpperHex](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html)
* [x] [Pointer](https://doc.rust-lang.org/std/fmt/trait.Pointer.html)
* [x] [Binary](https://doc.rust-lang.org/std/fmt/trait.Binary.html)
* [x] [LowerExp](https://doc.rust-lang.org/std/fmt/trait.LowerExp.html)
* [x] [UpperExp](https://doc.rust-lang.org/std/fmt/trait.UpperExp.html)

## Example
```rust
use owo_colors::OwoColorize;
 
fn main() {
    // Foreground colors
    println!("My number is {:#x}!", 10.green());
    // Background colors
    println!("My number is not {}!", 4.on_red());
}
```

## Generic colors
```rust
use owo_colors::OwoColorize;
use owo_colors::colors::*;

fn main() {
    // Generically color
    println!("My number might be {}!", 4.fg::<Black>().bg::<Yellow>());
}
```

## Stylize
```rust
use owo_colors::OwoColorize;

println!("{}", "strikethrough".strikethrough());
```
