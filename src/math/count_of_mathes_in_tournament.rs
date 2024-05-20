pub fn number_of_matches(mut n: i32) -> i32 {
    let mut result = 0;
    while n > 1 {
        let has_odd_team_number = n % 2 == 1;
        n >>= 1;
        result += n;

        if has_odd_team_number {
            n += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::math::count_of_mathes_in_tournament::number_of_matches;

    #[test]
    fn test_number_of_mathes() {
        assert_eq!(number_of_matches(7), 6);
    }
}
