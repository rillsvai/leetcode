use std::collections::HashMap;

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut count: u16 = 0;
    let length: usize = nums.len();

    for i in 0..length.checked_sub(1).unwrap_or(0) {
        for j in i + 1..length {
            if nums[i] == nums[j] {
                count += 1;
            }
        }
    }

    count as i32
}

pub fn num_identical_pairs2(nums: Vec<i32>) -> i32 {
    let mut pair_counts: HashMap<i32, u16> = HashMap::new();
    let mut count = 0;

    for num in nums {
        if !pair_counts.contains_key(&num) {
            pair_counts.insert(num, 0);
            continue;
        }

        pair_counts.insert(num, pair_counts[&num] + 1);
        count += pair_counts[&num];
    }

    count as i32
}

pub fn num_identical_pairs3(nums: Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, u16> = HashMap::new();
    let mut result: u16 = 0;

    for &num in &nums {
        let count = counts.entry(num).or_insert(0);
        result += *count;
        *count += 1;
    }

    result as i32
}
#[cfg(test)]
mod test {
    use crate::arrays::number_of_good_pairs::{
        num_identical_pairs, num_identical_pairs2, num_identical_pairs3,
    };

    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(num_identical_pairs(vec![]), 0);
        assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
    }

    #[test]
    fn test_num_identical_pairs2() {
        assert_eq!(num_identical_pairs2(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(num_identical_pairs2(vec![]), 0);
        assert_eq!(num_identical_pairs2(vec![1, 2, 3]), 0);
        assert_eq!(num_identical_pairs2(vec![1, 1, 1, 1]), 6);
    }

    #[test]
    fn test_num_identical_pairs3() {
        assert_eq!(num_identical_pairs3(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(num_identical_pairs3(vec![]), 0);
        assert_eq!(num_identical_pairs3(vec![1, 2, 3]), 0);
        assert_eq!(num_identical_pairs3(vec![1, 1, 1, 1]), 6);
    }
}
