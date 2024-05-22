use crate::common::list_node::ListNode;

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res: Box<ListNode> = Box::new(ListNode::new(0));
    let mut cur_node = &mut res;

    let mut head = head.as_ref();

    while let Some(node) = head {
        head = node.next.as_ref();

        if node.val == 0 && node.next != None {
            cur_node.next = Some(Box::new(ListNode::new(0)));
            cur_node = cur_node.next.as_mut().unwrap();

            continue;
        }

        cur_node.val += node.val;
    }

    res.next
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_merge_nodes() {
        assert!(true);
    }
}
