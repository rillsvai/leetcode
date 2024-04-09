pub fn min_partitions(n: String) -> i32 {
    n.chars()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::strings::partitioning_into_minimum_number_of_deci_binary_numbers::min_partitions;

    #[test]
    fn test_min_partitions() {
        assert_eq!(min_partitions("32".to_string()), 3);
        assert_eq!(min_partitions("82734".to_string()), 8);
        assert_eq!(min_partitions("27346209830709182346".to_string()), 9);
    }
}
