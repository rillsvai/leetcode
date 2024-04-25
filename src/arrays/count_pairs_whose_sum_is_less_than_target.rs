pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let length = nums.len();
    let mut count = 0;
    for i in 0..length - 1 {
        for j in i + 1..length {
            if nums[i] + nums[j] < target {
                count += 1;
            }
        }
    }

    count
}

pub fn count_pairs2(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut count = 0;

    while left < right {
        if nums[left] + nums[right] < target {
            count += right - left;
            left += 1;
        } else {
            right -= 1;
        }
    }

    count as i32
}
#[cfg(test)]
mod tests {
    use crate::arrays::count_pairs_whose_sum_is_less_than_target::{count_pairs, count_pairs2};

    #[test]
    fn test_count_pairs() {
        assert_eq!(count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10)
    }

    #[test]
    fn test_count_pairs2() {
        assert_eq!(count_pairs2(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(count_pairs2(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10)
    }
}
