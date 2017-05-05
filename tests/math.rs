extern crate seam_carving_resize;

use seam_carving_resize::math::*;

#[test]
fn wrap_to_bounds_test() {
    assert_eq!(9, wrap_to_bounds(-1, 0, 10));
    assert_eq!(12, wrap_to_bounds(2, 10, 20));
    assert_eq!(-5, wrap_to_bounds(-15, -10, 0));
}
