use hello_world_in_rust_derive::hello_world;

#[test]
fn compile_derive() {
    // This test should compile successfully
    #[hello_world]
    fn my_func() {}
    my_func();
}
