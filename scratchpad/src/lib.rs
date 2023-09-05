#[test]
fn test_adder() {
    use insta;
    let result = adder::add(1, 2);

    insta::assert_debug_snapshot!(result, @"3");
}
