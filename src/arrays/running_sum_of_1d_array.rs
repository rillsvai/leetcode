pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;

    for num in nums.iter_mut() {
        sum += *num;
        *num = sum;
    }

    nums
}
