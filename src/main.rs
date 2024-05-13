
fn main() {
  println!("hello {}!", hello_world::hello());
}

#[test]
fn test_main() {
  main();
}