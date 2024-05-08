pub struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let result = word.find(|letter| letter == ch);

        if let Some(end) = result {
            return word[0..=end].chars().rev().collect::<String>() + &word[(end + 1)..];
        }

        word
    }
}

#[cfg(test)]
mod tests {
    use crate::strings::reverse_prefix_of_word::Solution;

    #[test]
    fn test_reverse_prefix() {
        assert_eq!(
            Solution::reverse_prefix(String::from("abcdefd"), 'd'),
            String::from("dcbaefd")
        );
    }
}
