pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; nums.len()];

    for (i, _) in nums.iter().enumerate() {
        result[i] = nums[nums[i] as usize]; // Directly assign values to `result`
    }

    result
}

pub fn build_array2(mut nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len() as i32;

    for i in 0..length as usize {
        nums[i] += (nums[nums[i] as usize] % length) * length;
    }
    for i in 0..length as usize {
        nums[i] /= length;
    }

    nums
}

pub fn build_array3(mut nums: Vec<i32>) -> Vec<i32> {
    let mask: u16 = 0b1111111111;
    let shift: u8 = 10;
    let length = nums.len();

    for i in 0..length {
        nums[i] |= (nums[nums[i] as usize] & mask as i32) << shift;
    }
    for i in 0..length {
        nums[i] >>= shift;
    }

    nums
}
#[cfg(test)]
mod tests {

    use crate::arrays::build_array_from_permutation::{build_array, build_array2, build_array3};

    #[test]
    fn test_build_array() {
        assert_eq!(build_array(vec![5, 0, 1, 2, 3, 4]), [4, 5, 0, 1, 2, 3]);
    }

    #[test]
    fn test_buiild_array2() {
        assert_eq!(build_array2(vec![5, 0, 1, 2, 3, 4]), [4, 5, 0, 1, 2, 3]);
        let range = 0..=1499;
        assert_eq!(
            build_array2(range.clone().rev().collect()),
            range.collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_buiild_array3() {
        assert_eq!(build_array3(vec![5, 0, 1, 2, 3, 4]), [4, 5, 0, 1, 2, 3]);
        let range = 0..=1000;
        assert_eq!(
            build_array3(range.clone().rev().collect()),
            range.collect::<Vec<_>>()
        );
    }
}
