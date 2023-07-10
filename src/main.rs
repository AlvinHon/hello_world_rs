
fn main() {
  println!("hello {}!", hello_world_rs::hello());
}

#[test]
fn test_main() {
  main();
}