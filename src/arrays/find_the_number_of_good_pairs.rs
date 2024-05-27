pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;
    for num1 in nums1.iter() {
        for num2 in nums2.iter() {
            if num1 % (num2 * k) == 0 {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::arrays::{find_the_number_of_good_pairs::number_of_pairs, number_of_good_pairs};

    #[test]
    fn test_number_of_pairs() {
        assert_eq!(number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3), 2)
    }
}
