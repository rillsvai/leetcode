pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().fold(k, |acc, num| acc ^ num).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays::minimum_number_of_operations_to_make_array_xor_equal_to_k::Solution;

    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 1, 3, 4], 1), 2);
        assert_eq!(Solution::min_operations(vec![2, 0, 2, 0], 0), 0);
    }
}
