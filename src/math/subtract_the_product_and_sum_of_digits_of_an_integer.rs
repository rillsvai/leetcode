pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut accumulator: (i32, i32) = (1, 0);

    while n != 0 {
        let digit = n % 10;
        n /= 10;

        accumulator.0 *= digit;
        accumulator.1 += digit;
    }

    accumulator.0 - accumulator.1
}

#[cfg(test)]
mod tests {
    use crate::math::subtract_the_product_and_sum_of_digits_of_an_integer::subtract_product_and_sum;

    #[test]
    fn test_subtract_product_and_sum() {
        assert_eq!(subtract_product_and_sum(234), 15);
        assert_eq!(subtract_product_and_sum(4421), 21);
    }
}
