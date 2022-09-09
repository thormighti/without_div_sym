use without_div_sym::divide;

#[test]

fn test_divide() {
    assert_eq!(Some(10), divide(50, 5));
}
