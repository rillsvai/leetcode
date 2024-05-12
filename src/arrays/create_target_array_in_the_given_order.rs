pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());

    for i in 0..nums.len() {
        result.insert(index[i] as usize, nums[i]);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::arrays::create_target_array_in_the_given_order::create_target_array;

    #[test]
    fn test_create_target_array() {
        assert_eq!(
            create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
    }
}
