use std::ops::Rem;

use crate::common::list_node::ListNode;

pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut iter = head.as_deref_mut();

    while let Some(current) = iter {
        if let Some(next) = current.next.as_deref() {
            let gcd = gcd(current.val, next.val);

            current.next = Some(Box::new(ListNode {
                val: gcd,
                next: current.next.take(),
            }));

            iter = current
                .next
                .as_deref_mut()
                .and_then(|gcd_node| gcd_node.next.as_deref_mut());
        } else {
            break;
        }
    }

    head
}

pub fn insert_greatest_common_divisors2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head.as_deref_mut();

    while let Some(node) = current {
        if node.next.is_none() {
            break;
        }
        let mut gcd_node = ListNode::new(gcd(node.val, node.next.as_ref().unwrap().val));

        gcd_node.next = node.next.take();
        node.next = Some(Box::new(gcd_node));

        current = node
            .next
            .as_deref_mut()
            .and_then(|gcd| gcd.next.as_deref_mut());
    }
    head
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
    use crate::{
        common::list_node::ListNode,
        math::insert_greatest_common_divisors_in_linked_list::{
            insert_greatest_common_divisors, insert_greatest_common_divisors2,
        },
    };

    use super::gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 6), 6);
        assert_eq!(gcd(10, 3), 1);
    }

    #[test]
    fn test_insert_greatest_common_divisors() {
        let head = Some(Box::new(ListNode {
            val: 18,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 10,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));

        let result = Some(Box::new(ListNode {
            val: 18,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 10,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode { val: 3, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(insert_greatest_common_divisors(head), result);
    }

    #[test]
    fn test_insert_greatest_common_divisors2() {
        let head = Some(Box::new(ListNode {
            val: 18,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 10,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));

        let result = Some(Box::new(ListNode {
            val: 18,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 10,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode { val: 3, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(insert_greatest_common_divisors2(head), result);
    }
}
