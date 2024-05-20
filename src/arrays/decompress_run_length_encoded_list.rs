pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    nums.chunks(2)
        .for_each(|pair| result.append(&mut vec![pair[1]; pair[0] as usize]));

    result
}

#[cfg(test)]
mod tests {
    use crate::arrays::decompress_run_length_encoded_list::decompress_rl_elist;

    #[test]
    fn test_decompress_rl_elist() {
        assert_eq!(decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
    }
}
