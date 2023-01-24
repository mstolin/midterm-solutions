pub fn maybediv(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        return None;
    }
    Some(dividend / divisor)
}
