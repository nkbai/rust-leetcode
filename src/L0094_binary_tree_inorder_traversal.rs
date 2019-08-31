/*
 94. 二叉树的中序遍历
 给定一个二叉树，返回它的中序 遍历。

示例:

输入: [1,null,2,3]
   1
    \
     2
    /
   3

输出: [1,3,2]
进阶: 递归算法很简单，你可以通过迭代算法完成吗？



来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-inorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Solution::inorder_internal(root, &mut v);
        v
    }

    fn inorder_internal(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(root) = root {
            Solution::inorder_internal(root.borrow().left.clone(), v);
            v.push(root.borrow().val);
            Solution::inorder_internal(root.borrow().right.clone(), v);
        }
    }
    pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Solution::inorder_non_recursive(root, &mut v);
        v
    }
    /*
        将指定节点从自身到最深的最左边path压栈
    //! ```text
    //!      3
    //!     /  \
    //!    /    \
    //!   1      5 <-[Node index, a.k.a, Position]
    //!  / \    / \
    //! 0   2  4   6
    //!
    //! 0   1  2   3 <[Leaf index]
    //! ```
        比如,传递进来3,
        那么就会压栈[3,1,0]
        */
    fn pushNodes(mut root: Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<Rc<RefCell<TreeNode>>>) {
        while let Some(r) = root {
            stack.push(r.clone());
            root = r.borrow().left.clone();
        }
        return;
    }
    /*
    思路, 首先将最左边整条路径压栈
    然后取出来一个,打印,然后尝试访问
    */
    fn inorder_non_recursive(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        let mut nodes = Vec::new();
        Solution::pushNodes(root, &mut nodes);
        //while let 显然更rust
        while let Some(n) = nodes.pop() {
            v.push(n.borrow().val);
            /*
            压栈右子树,如果有的话
            */
            Solution::pushNodes(n.borrow().right.clone(), &mut nodes);
        }
        return;
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
        let t2 = t.unwrap();
        assert_eq!(Solution::inorder_traversal(Some(t2.clone())), vec![1, 3, 2]);
        assert_eq!(
            Solution::inorder_traversal2(Some(t2.clone())),
            vec![1, 3, 2]
        );
    }
}
