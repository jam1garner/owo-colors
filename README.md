# <img src="https://jam1.re/img/rust_owo.svg" height="100"> Colors
[![Current Crates.io Version](https://img.shields.io/crates/v/owo-colors.svg)](https://crates.io/crates/owo-colors)
[![docs-rs](https://docs.rs/owo-colors/badge.svg)](https://docs.rs/owo-colors)
![MSRV 1.34+](https://img.shields.io/badge/rustc-1.34+-blue.svg)

A zero-allocation no_std-compatible zero-cost way to add color to your Rust terminal to make people go owo.

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

owo-colors is also more-or-less a drop-in replacement for [colored](https://crates.io/crates/colored), allowing colored to work in a no_std environment. No allocations, unsafe, or dependencies required because embedded systems deserve to be pretty too uwu.

To add to your Cargo.toml:
```toml
owo-colors = "1"
```

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
