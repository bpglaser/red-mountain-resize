// Wraps the value to the range [min, max)
pub fn wrap_to_bounds(mut value: isize, min: isize, max: isize) -> isize {
    let range = max - min;
    if value < min {
        value += range * ((min - value) / range + 1);
    }
    min + (value - min) % range
}

#[test]
fn wrap_to_bounds_test() {
    assert_eq!(9, wrap_to_bounds(-1, 0, 10));
    assert_eq!(12, wrap_to_bounds(2, 10, 20));
    assert_eq!(-5, wrap_to_bounds(-15, -10, 0));
}
