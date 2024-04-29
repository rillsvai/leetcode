use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn get_test_tree() -> Rc<RefCell<TreeNode>> {
        let root = Rc::new(RefCell::new(TreeNode::new(4)));
        let node8 = Rc::new(RefCell::new(TreeNode::new(8)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node0 = Rc::new(RefCell::new(TreeNode::new(0)));
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));

        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.left = Some(node8.clone());
            root_borrowed.right = Some(node5.clone());
        }

        {
            let mut node8_borrowed = node8.borrow_mut();
            node8_borrowed.left = Some(node0.clone());
            node8_borrowed.right = Some(node1.clone());
        }

        {
            let mut node5_borrowed = node5.borrow_mut();
            node5_borrowed.right = Some(node6.clone());
        }

        root
    }
}
