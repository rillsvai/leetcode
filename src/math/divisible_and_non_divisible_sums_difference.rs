pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut diff: i32 = 0;

    for i in 1..=n {
        if i % m != 0 {
            diff += i;
            continue;
        }
        diff -= i;
    }

    diff
}

pub fn difference_of_sums2(n: i32, m: i32) -> i32 {
    (1..=n).map(|i| if i % m == 0 { -i } else { i }).sum()
}

pub fn difference_of_sums3(n: i32, m: i32) -> i32 {
    let total_sum = n * (n + 1) / 2;
    let divisible_count: i32 = n / m;
    let divisible_sum: i32 = (divisible_count * (divisible_count + 1) / 2) * m;
    total_sum - 2 * divisible_sum
}
#[cfg(test)]
mod tests {
    use crate::math::divisible_and_non_divisible_sums_difference::{
        difference_of_sums, difference_of_sums2, difference_of_sums3,
    };

    #[test]
    fn test_difference_of_sums() {
        assert_eq!(difference_of_sums(10, 3), 19);
        assert_eq!(difference_of_sums(5, 6), 15);
        assert_eq!(difference_of_sums(5, 1), -15);
    }

    #[test]
    fn test_difference_of_sums2() {
        assert_eq!(difference_of_sums2(10, 3), 19);
        assert_eq!(difference_of_sums2(5, 6), 15);
        assert_eq!(difference_of_sums2(5, 1), -15);
    }

    #[test]
    fn test_difference_of_sums3() {
        assert_eq!(difference_of_sums3(10, 3), 19);
        assert_eq!(difference_of_sums3(5, 6), 15);
        assert_eq!(difference_of_sums3(5, 1), -15);
    }
}
