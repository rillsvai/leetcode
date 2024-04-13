use std::ops::Rem;

use crate::common::list_node::ListNode;

pub fn insert_greatest_common_divisors(mut _head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!();
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + Copy + PartialEq<i32>,
{
    let mut a = a;
    let mut b = b;
    let mut remainder = a % b;

    while remainder != 0 {
        (a, b) = (b, remainder);
        remainder = a % b;
    }

    b
}
#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 6), 6);
        assert_eq!(gcd(10, 3), 1);
    }

    #[test]
    fn test_insert_greatest_common_divisors() {
        assert!(true);
    }
}
