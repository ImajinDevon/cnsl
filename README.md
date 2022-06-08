# cnsl

`cnsl` is a crate that provides methods and macros for interacting with the command line.

![Rust Build Status](https://github.com/ImajinDevon/cnsl/actions/workflows/rust.yml/badge.svg)

---

# Usage

## readln macro

```rust
// without a prompt
```
```rust
use cnsl::readln;

fn main() {
    println!("What is your name?");
    let name = readln!();
    println!("Hello, {}!", name);
}
```
---
```rust
// with a prompt
```
```rust
use cnsl::readln;

fn main() {
    let name = readln!("Enter your name: ");
    println!("Hello, {}!", name);
}
```
---
```rust
// with a formatted prompt
// note: defaults are not actually supported, this is just a demonstration
```
```rust
use cnsl::readln;

const DEFAULT_AGE: u8 = 18;

fn main() {
    let age_input = readln!("Enter your age (default {}): ", DEFAULT_AGE);
    
    let age = if age_input.is_empty() {
        DEFAULT_AGE
    } else {
        age_input.parse::<u8>().expect("invalid input for age")
    };
}
```

# Information

## License

This software is licensed under the [WTFPL](https://www.wtfpl.net/).

## Contributors

- [ImajinDevon](https://www.github.com/imajindevon/)

---
<footer>© 2022 WTFPL – Do What the Fuck You Want to Public License.</footer>