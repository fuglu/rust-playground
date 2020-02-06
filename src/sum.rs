pub fn sum(a: i32, b: i32) -> i32 {
    return a+b;
}

#[test]
fn testSum() {
    assert!(sum(1, 2) == 3);
}
