use hello;
#[test]
fn hello_integration_test() {
    assert_eq!("Hello! User", hello::greeting("User"))
}
