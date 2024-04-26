use std::collections::HashMap;

pub fn smaller_numbers_than_current(mut nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    let mut counts = HashMap::<i32, usize>::new();

    for (i, num) in sorted_nums.iter().enumerate() {
        if !counts.contains_key(num) {
            counts.insert(*num, i);
        }
    }

    for i in 0..nums.len() {
        nums[i] = counts[&nums[i]] as i32;
    }

    nums
}

#[cfg(test)]
mod tests {
    use crate::arrays::how_many_numbers_are_smaller_than_the_current_number::smaller_numbers_than_current;

    #[test]
    fn test_smaller_numbers_than_current() {
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );

        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );

        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
