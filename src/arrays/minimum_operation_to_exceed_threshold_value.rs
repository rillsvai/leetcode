pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    nums.iter().position(|num| *num >= k).unwrap() as i32
}
