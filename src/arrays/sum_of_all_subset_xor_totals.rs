fn subset_xor_sum_call(nums: &Vec<i32>, i: usize, current_xor: i32) -> i32 {
    if i == nums.len() {
        return current_xor;
    }

    subset_xor_sum_call(nums, i + 1, current_xor)
        + subset_xor_sum_call(nums, i + 1, current_xor ^ nums[i])
}

pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    subset_xor_sum_call(&nums, 0, 0)
}

#[cfg(test)]
mod tests {
    use crate::arrays::sum_of_all_subset_xor_totals::subset_xor_sum;

    fn test_subset_xor_sum() {
        assert_eq!(subset_xor_sum(vec![1, 3]), 6);
        assert_eq!(subset_xor_sum(vec![5, 1, 6]), 28);
        assert_eq!(subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480)
    }
}
