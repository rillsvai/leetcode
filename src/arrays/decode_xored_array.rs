pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut result = vec![0; encoded.len() + 1];

    result[0] = first;

    for i in 0..encoded.len() {
        result[i + 1] = encoded[i] ^ result[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::arrays::decode_xored_array::decode;

    #[test]
    fn test_decode() {
        assert_eq!(decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
        assert_eq!(decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4])
    }
}
