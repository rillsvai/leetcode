pub struct Solution {}

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];

        let sum: i32 = nums.iter().sum();
        let mut right_sum = sum - nums[0];
        let mut left_sum = 0;

        result[0] = right_sum;

        for i in 1..nums.len() {
            left_sum += nums[i - 1];
            right_sum -= nums[i];
            result[i] = left_sum.abs_diff(right_sum) as i32;
        }

        result
    }

    pub fn left_right_difference2(mut nums: Vec<i32>) -> Vec<i32> {
        let mut right_sum: i32 = nums.iter().sum();
        let mut left_sum: i32 = 0;
        let mut previous_element;

        for i in 0..nums.len() {
            right_sum -= nums[i];
            previous_element = nums[i];
            nums[i] = left_sum.abs_diff(right_sum) as i32;
            left_sum += previous_element;
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays::left_and_right_sum_differences::Solution;

    #[test]
    fn test_left_right_difference() {
        assert_eq!(
            Solution::left_right_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
    }

    #[test]
    fn test_left_right_difference2() {
        assert_eq!(
            Solution::left_right_difference2(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
    }
}
