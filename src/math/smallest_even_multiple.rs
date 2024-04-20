pub fn smallest_even_multiple(n: i32) -> i32 {
    if n % 2 == 0 {
        return n;
    }
    n * 2
}

#[cfg(test)]
mod tests {
    use crate::math::smallest_even_multiple::smallest_even_multiple;

    #[test]
    fn test_smallest_even_multiple() {
        assert_eq!(smallest_even_multiple(5), 10);
        assert_eq!(smallest_even_multiple(7), 14);
        assert_eq!(smallest_even_multiple(4), 4);
        assert_eq!(smallest_even_multiple(6), 6);
    }
}
