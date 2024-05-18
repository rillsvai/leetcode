use std::{cell::RefCell, rc::Rc};

use crate::common::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, value: &mut i32) {
        if let Some(node) = root {
            let mut borrowed_node = node.borrow_mut();
            Self::dfs(&borrowed_node.right, value);
            *value += borrowed_node.val;
            borrowed_node.val = *value;
            Self::dfs(&borrowed_node.left, value);
        }
    }
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root, &mut 0);
        root
    }
}
