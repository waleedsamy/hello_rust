use hello::greetings;
#[test]
fn hello_integration_test() {
    assert_eq!("Hello! User", greetings::greeting("User"))
}
