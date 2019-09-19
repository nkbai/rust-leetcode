/*
https://leetcode-cn.com/problems/flatten-binary-tree-to-linked-list/
114. 二叉树展开为链表
给定一个二叉树，原地将它展开为链表。

例如，给定二叉树

    1
   / \
  2   5
 / \   \
3   4   6
将其展开为：

1
 \
  2
   \
    3
     \
      4
       \
        5
         \
          6


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/flatten-binary-tree-to-linked-list
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
用了rust,确实会让思路清晰,编译过了,结果就对了.
*/
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
struct Solution {}
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let pre = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        Solution::flattern_internal(pre.unwrap(), root.clone());
    }
    fn flattern_internal(
        root: Rc<RefCell<TreeNode>>,
        cur: Option<Rc<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<TreeNode>> {
        //        println!("root={:?},cur={:?}", root, cur);
        if let Some(r) = cur {
            root.borrow_mut().right = Some(r.clone());
            let root = root.borrow().right.clone().unwrap();
            let left = r.borrow_mut().left.take();
            let right = r.borrow_mut().right.take();
            //            println!("root={:?}", root);
            let root = Solution::flattern_internal(root, left);
            //            println!("rootleft={:?}", root);
            let root = Solution::flattern_internal(root, right);
            //            println!("root right={:?}", root);
            return root;
        }
        root
    }
}
#[cfg(test)]
mod tests {
    use crate::l114_flattern_binary_tree_to_linked_list::Solution;
    use crate::share::build_tree;
    use crate::share::NULL;

    #[test]
    fn test_flattern() {
        let mut t = build_tree(&vec![1, 2, 5, 3, 4, NULL, 6]);
        println!("before t={:?}", t);
        Solution::flatten(&mut t);
        println!("t={:?}", t);
    }
}
