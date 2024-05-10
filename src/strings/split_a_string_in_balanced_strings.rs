pub fn balanced_string_split(s: String) -> i32 {
    let mut result = 0;
    let mut balance_count = 0;
    for ch in s.chars() {
        balance_count += match ch {
            'R' => 1,
            'L' => -1,
            _ => 0,
        };

        if balance_count == 0 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::strings::split_a_string_in_balanced_strings::balanced_string_split;

    #[test]
    fn test_balanced_string_split() {
        assert_eq!(balanced_string_split(String::from("RLRRLLRLRL",)), 4);
        assert_eq!(balanced_string_split(String::from("RLRRRLLRLL")), 2);
        assert_eq!(balanced_string_split(String::from("LLLLRRRR")), 1);
    }
}
