pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut result: i32 = 0;

    for operation in operations {
        if operation.contains("++") {
            result += 1;
            continue;
        }
        result -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::strings::final_value_of_variable_after_performing_operations::final_value_after_operations;

    #[test]
    fn test_final_value_after_operations() {
        assert_eq!(
            final_value_after_operations(["--X", "X++", "X++"].map(|s| s.to_string()).to_vec()),
            1
        );

        assert_eq!(
            final_value_after_operations(
                ["X++", "++X", "--X", "X--"].map(|s| s.to_string()).to_vec()
            ),
            0
        )
    }
}
