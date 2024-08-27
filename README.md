# stringvec

![ci](https://github.com/bednie/stringvec/actions/workflows//badge.svg)

A simple Rust macro for creating `Vec<String>` from various types.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
stringvec = "0.1.0"
```

Then, in your Rust code:

```rust
use stringvec::stringvec;

fn main() {
    let words = stringvec!["cat", 11, 'A', 3.14];
    println!("{:?}", words);
    // Output: ["cat", "11", "A", "3.14"]
}
```

## Features

- Easy creation of `Vec<String>` from various types
- Automatic conversion to `String` using `to_string()`
- Utility function `is_string()` to check if a value is a `String`

## License

This project is licensed under the GNU Lesser General Public License v3.0 or later - see the [LICENSE](LICENSE) file for details.