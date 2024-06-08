//! This program prints "hello world!!" on console.

fn main() {
    println!("hello {}!", hello_world::hello());
}

#[test]
fn test_main() {
    // expected to run without panics
    main();
}
