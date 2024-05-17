pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut result = 0;

    for i in 0..n {
        result ^= start + 2 * i;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::arrays::xor_operation_in_a_array::xor_operation;

    #[test]
    fn test_xor_operation() {
        assert_eq!(xor_operation(5, 0), 8);
        assert_eq!(xor_operation(4, 3), 8);
    }
}
