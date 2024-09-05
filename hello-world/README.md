[![version]][crates.io] [![workflow status]][workflow] [![codecov status]][codecov]

[version]: https://img.shields.io/crates/v/hello-world-in-rust.svg
[crates.io]: https://crates.io/crates/hello-world-in-rust
[workflow status]: https://github.com/AlvinHon/hello_world_rs/actions/workflows/rust.yml/badge.svg?branch=main
[workflow]: https://github.com/AlvinHon/hello_world_rs/actions/workflows/rust.yml
[codecov status]: https://codecov.io/gh/AlvinHon/hello_world_rs/branch/main/graph/badge.svg?token=NMM473N4DO
[codecov]: https://codecov.io/gh/AlvinHon/hello_world_rs

# Hello World

Hello world in Rust!

Run the binary:

```bash
hello-world-in-rust
# Output: hello world!
```

Execute as crate:

```rust
let next_word = hello_world::hello();
```

## Hello World Derive

Add macro to print out the hello world message in a function.

```rust
#[hello_world]
fn main {
    // The macro will generate println! to print the hello world message here.
}
```