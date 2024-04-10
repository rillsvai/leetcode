use std::collections::HashMap;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut occurences = HashMap::<char, u8>::new();

    for stone in stones.chars() {
        if jewels.contains(stone) {
            let entry = occurences.entry(stone).or_insert(0);
            *entry += 1;
        }
    }

    occurences.values().sum::<u8>() as i32
}

pub fn num_jewels_in_stones2(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .filter(|stone| jewels.contains(*stone))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use crate::strings::jewels_and_stones::{num_jewels_in_stones, num_jewels_in_stones2};

    #[test]
    fn test_num_jewels_in_stones() {
        assert_eq!(
            num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );

        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }

    #[test]
    fn test_num_jewels_in_stones2() {
        assert_eq!(
            num_jewels_in_stones2("aA".to_string(), "aAAbbbb".to_string()),
            3
        );

        assert_eq!(num_jewels_in_stones2("z".to_string(), "ZZ".to_string()), 0);
    }
}
