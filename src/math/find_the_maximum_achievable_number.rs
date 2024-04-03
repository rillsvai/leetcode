pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    num + t * 2
}

#[cfg(test)]
mod tests {
    use super::the_maximum_achievable_x;

    #[test]
    fn test_the_maximum_achievable_x() {
        assert_eq!(the_maximum_achievable_x(4, 1), 6);
        assert_eq!(the_maximum_achievable_x(3, 2), 7);
    }
}
