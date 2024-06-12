pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut result = vec![0; nums.len()];

    for num in nums.iter() {
        if *num < pivot {
            result[left] = *num;
            left += 1;
        }
    }

    for num in nums.iter().rev() {
        if *num > pivot {
            result[right] = *num;
            right -= 1;
        }

        if *num == pivot {
            result[left] = *num;
            left += 1;
        }
    }

    result
}
