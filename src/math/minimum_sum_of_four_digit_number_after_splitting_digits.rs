pub struct Solution {}

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits: Vec<u32> = num
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect();
        digits.sort();

        let sum = (digits[0] + digits[1]) * 10 + digits[2] + digits[3];

        sum as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::math::minimum_sum_of_four_digit_number_after_splitting_digits::Solution;

    #[test]
    fn test_minimum_sum() {
        assert_eq!(Solution::minimum_sum(2932), 52);
        assert_eq!(Solution::minimum_sum(4009), 13);
    }
}
