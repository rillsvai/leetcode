pub fn is_strictly_palindromic(_n: i32) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::math::strictly_palindromic_number::is_strictly_palindromic;

    #[test]
    fn test_is_strictly_palindromic() {
        assert_eq!(is_strictly_palindromic(9), false);
        assert_eq!(is_strictly_palindromic(4), false)
    }
}
