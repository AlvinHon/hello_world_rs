#[test]
fn test_hello_world() {
  let what = hello_world::hello();
  assert_eq!(what, "world!".to_string());
}
