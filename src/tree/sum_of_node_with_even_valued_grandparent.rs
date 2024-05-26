use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;
pub struct Solution;

impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, grand_parent: i32, parent: i32) -> i32 {
        match root {
            Some(root_ref) => {
                let root_node = root_ref.borrow();
                let mut total = 0;

                if grand_parent % 2 == 0 {
                    total += root_node.val;
                }

                total += Self::dfs(&root_node.left, parent, root_node.val);
                total += Self::dfs(&root_node.right, parent, root_node.val);

                total
            }
            None => 0,
        }
    }

    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 1, 1)
    }
}
