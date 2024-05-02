pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;

    for (i, num) in nums.iter().enumerate() {
        if i.count_ones() == k as u32 {
            sum += num;
        }
    }

    sum
}

pub fn sum_indices_with_k_set_bits2(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;

    for i in 0..nums.len() {
        let mut bit_count = 0;

        let mut temp = i;
        while temp != 0 {
            bit_count += temp & 1;
            temp >>= 1;
        }

        if bit_count == k as usize {
            sum += nums[i];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::arrays::sum_of_values_at_indices_with_k_set_bits::{
        sum_indices_with_k_set_bits, sum_indices_with_k_set_bits2,
    };

    #[test]
    fn test_sum_indices_with_k_set_bits() {
        assert_eq!(sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1), 13);
        assert_eq!(sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2), 1);
    }

    #[test]
    fn test_sum_indices_with_k_set_bits2() {
        assert_eq!(sum_indices_with_k_set_bits2(vec![5, 10, 1, 5, 2], 1), 13);
        assert_eq!(sum_indices_with_k_set_bits2(vec![4, 3, 2, 1], 2), 1);
    }
}
