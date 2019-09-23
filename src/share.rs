// Definition for singly-linked list.
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
pub const NULL: i32 = std::i32::MIN;
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Ord for ListNode {
    fn cmp(&self, other: &ListNode) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub fn build_list_node(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut();
    for i in v {
        let next = ListNode::new(*i);
        let v = current.unwrap();
        //        let Some(v) = current;
        v.next = Some(Box::new(next));
        current = v.next.as_mut();
    }
    //    println!("head={:#?}",head);
    head.unwrap().next
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
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
}
/*
构造树的时候,按照leetcode的规则来,如果父节点是空,那么就不必 给出子节点.
这样就不是严格意义上的那个2i+1,2i+2,稍微复杂一点
*/
pub fn build_tree_ignore_parent(v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    None
}

pub fn build_tree(v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_helper(0, v)
}
fn build_tree_helper(i: usize, v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if i >= v.len() {
        return None;
    }
    if v[i] == NULL {
        return None;
    }
    let left = build_tree_helper(2 * i + 1, v);
    let right = build_tree_helper(2 * i + 2, v);
    Some(Rc::new(RefCell::new(TreeNode {
        val: v[i],
        left,
        right,
    })))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_build_tree() {
        println!("tree={:?}", build_tree(&vec![0, -3, 9, -10, NULL, 5]));
    }
}
