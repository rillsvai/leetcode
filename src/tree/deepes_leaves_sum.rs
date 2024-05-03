use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut sum = 0;
        queue.push_back(root.unwrap().clone());

        while !queue.is_empty() {
            sum = 0;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let borrowed_node = node.borrow();
                sum += borrowed_node.val;

                if let Some(right) = borrowed_node.right.clone() {
                    queue.push_back(right);
                }

                if let Some(left) = borrowed_node.left.clone() {
                    queue.push_back(left);
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node::TreeNode;

    #[test]
    fn test_deepest_leaves_sum() {
        let tree = TreeNode::get_test_tree();

        assert_eq!(Solution::deepest_leaves_sum(Some(tree)), 7);
    }
}
