use std::{cell::RefCell, rc::Rc};

use crate::common::tree_node::TreeNode;

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(node) = root {
        let elem = node.borrow().val;
        let addition = if elem >= low && elem <= high { elem } else { 0 };
        return addition
            + range_sum_bst(node.borrow().left.clone(), low, high)
            + range_sum_bst(node.borrow().right.clone(), low, high);
    }

    0
}
