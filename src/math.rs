// Wraps the value to the range [min, max)
pub fn wrap_to_bounds(mut value: isize, min: isize, max: isize) -> isize {
    let range = max - min;
    if value < min {
        value += range * ((min - value) / range + 1);
    }
    min + (value - min) % range
}
