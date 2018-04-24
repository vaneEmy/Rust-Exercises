pub fn hello() -> String{
    ("Hello, world!").to_string()
}

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello, wolrd!");
}