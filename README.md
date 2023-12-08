# minigrep

This project is based on [The Rust Book: Section 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) project. Which is a minimal implementation of the grep CLI tool.

## To run the application

``` bash
cargo run -- frog poem.txt              # Single match
cargo run -- body poem.txt              # Multiple match
cargo run -- monomorphization poem.txt  # No match
```
