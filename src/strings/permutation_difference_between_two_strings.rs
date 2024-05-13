use std::collections::HashMap;

pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut s_positions: HashMap<char, usize> = HashMap::new();
    let mut result: i32 = 0;

    s.chars().enumerate().for_each(|(i, ch)| {
        s_positions.insert(ch, i);
    });

    for (i, ch) in t.chars().enumerate() {
        result += (s_positions[&ch] as i32).abs_diff(i as i32) as i32;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::strings::permutation_difference_between_two_strings::find_permutation_difference;

    #[test]
    fn test_find_permutation_difference() {
        assert_eq!(
            find_permutation_difference(String::from("abc"), String::from("bac")),
            2
        );
        assert_eq!(
            find_permutation_difference(String::from("abcde"), String::from("edbac")),
            12
        )
    }
}
