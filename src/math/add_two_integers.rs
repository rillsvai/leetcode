pub fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

#[cfg(test)]
mod tests {
    use crate::math::add_two_integers::sum;

    #[test]
    fn test_sum() {
        assert_eq!(sum(4, -98), -94);
    }
}
