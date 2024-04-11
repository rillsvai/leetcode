use core::num;

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result: Vec<i32> = vec![0; 2 * n];

    for i in 0..n as usize {
        result[i % n << 1] = nums[i];
        result[(i % n << 1) + 1] = nums[i + n];
    }

    result
}

pub fn shuffle2(mut nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;

    let mask = 0b1111111111;
    let shift: u8 = 10;

    for i in 0..n as usize {
        nums[i + n] |= (nums[i]) << shift;
    }

    for i in 0..n as usize {
        nums[i << 1] = nums[i + n] >> shift;
        nums[(i << 1) + 1] = nums[i + n] & mask;
    }

    nums
}
#[cfg(test)]
mod tests {
    use crate::arrays::shuffle_the_array::{shuffle, shuffle2};

    #[test]
    fn test_shuffle() {
        assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
        assert_eq!(
            shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }

    #[test]
    fn test_shuffle2() {
        assert_eq!(shuffle2(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
        assert_eq!(
            shuffle2(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(shuffle2(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }
}
