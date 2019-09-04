/*
145. 二叉树的后序遍历
 给定一个二叉树，返回它的中序 遍历。

示例:

输入: [1,null,2,3]
   1
    \
     2
    /
   3

输出: [3,2,1]
进阶: 递归算法很简单，你可以通过迭代算法完成吗？

 https://leetcode-cn.com/problems/binary-tree-postorder-traversal/
*/
//use std::collections::HashMap;
struct Solution {}
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Solution::traversal(root, &mut v);
        return v;
    }
    fn traversal(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(r) = root {
            Solution::traversal(r.borrow().left.clone(), v);
            Solution::traversal(r.borrow().right.clone(), v);
            v.push(r.borrow().val);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_inorder_traversal() {
        let t = build_tree(&vec![1, NULL, 2, NULL, NULL, 3]);
        println!("t={:?}", t);
        assert_eq!(Solution::postorder_traversal(t), vec![3, 2, 1]);
    }
}
