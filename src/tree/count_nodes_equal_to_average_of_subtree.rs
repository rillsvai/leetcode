use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;

        if let Some(node) = root {
            let borrowed_node = node.borrow();

            let stats = Self::get_stats_of_tree(Some(node.clone()));

            if borrowed_node.val == stats.0 / stats.1 {
                count += 1;
            }

            count += Self::average_of_subtree(node.borrow().left.clone())
                + Self::average_of_subtree(node.borrow().right.clone());

            return count;
        }

        0
    }

    pub fn get_stats_of_tree(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        let mut stats = (0, 0);

        if let Some(node) = root {
            let borrowed_node = node.borrow();

            let left_result = Self::get_stats_of_tree(borrowed_node.left.clone());
            let right_result = Self::get_stats_of_tree(borrowed_node.right.clone());

            stats.0 += left_result.0 + right_result.0 + borrowed_node.val;
            stats.1 += right_result.1 + left_result.1 + 1;

            return stats;
        }

        (0, 0)
    }

    pub fn average_of_subtree2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn get_stats_of_tree2(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32) {
            let mut stats = (0, 0);

            if let Some(node) = root {
                let borrowed_node = node.borrow();

                let right_result = get_stats_of_tree2(borrowed_node.right.clone(), result);
                let left_result = get_stats_of_tree2(borrowed_node.left.clone(), result);

                stats.0 += left_result.0 + right_result.0 + borrowed_node.val;
                stats.1 += right_result.1 + left_result.1 + 1;

                if stats.0 / stats.1 == borrowed_node.val {
                    *result += 1;
                }

                return stats;
            }

            (0, 0)
        }

        let mut result = 0;
        get_stats_of_tree2(root, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::tree_node::TreeNode, tree::count_nodes_equal_to_average_of_subtree::Solution,
    };

    #[test]
    fn test_get_stats_of_tree() {
        let root = TreeNode::get_test_tree();

        let result = Solution::get_stats_of_tree(Some(root));
        assert_eq!(result.0, 24);
        assert_eq!(result.1, 6);
    }

    #[test]
    fn test_average_of_subtree() {
        let root = TreeNode::get_test_tree();
        assert_eq!(Solution::average_of_subtree(Some(root)), 5);
    }

    #[test]
    fn test_average_of_subtree2() {
        let root = TreeNode::get_test_tree();
        assert_eq!(Solution::average_of_subtree2(Some(root)), 5);
    }
}
