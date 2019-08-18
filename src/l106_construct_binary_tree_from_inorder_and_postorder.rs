/*
https://leetcode-cn.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
106. 从中序与后序遍历序列构造二叉树

根据一棵树的中序遍历与后序遍历构造二叉树。

注意:
你可以假设树中没有重复的元素。

例如，给出

中序遍历 inorder = [9,3,15,20,7]
后序遍历 postorder = [9,15,7,20,3]
返回如下的二叉树：

    3
   / \
  9  20
    /  \
   15   7

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路: 和第105题是一模一样的
首先题目中**假设树中没有重复的元素** 这句话非常关键,否则就无法构造了.
1. 在后序order中提取最后元素作为树根
2. 然后在中序遍历中依据这个树根将数组分为前后两部分,前半部分是左子树,后半部分是右子树.
3. 后序遍历中的倒数第二个元素就是树根的右子节点,右子节点-len(右子树)肯定是左子节点.
4. 递归回到1
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
use crate::share::{build_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_tree_internal(&postorder, &inorder)
    }
    fn build_tree_internal(postorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.len() == 0 {
            return None;
        }
        let (root, leftRight) = postorder.split_last().expect("must have one element");
        let rootNode = Rc::new(RefCell::new(TreeNode::new(*root)));
        let mut mid = -1;
        //在中序遍历中找root
        for i in 0..inorder.len() {
            if inorder[i] == *root {
                mid = i as i32;
                break;
            }
        }
        if mid == -1 {
            panic!("data wrong")
        }
        let (inorderLeft, inOrderRightContainsMid) = inorder.split_at(mid as usize);
        let (_, inOrderRight) = inOrderRightContainsMid
            .split_first()
            .expect("must have one element"); //剔除根节点
        let (postorderLeft, postorderRight) = leftRight.split_at(inorderLeft.len());
        //        println!(
        //            "root={},postorderLeft={:?},postorderRight={:?},inorderLeft={:?},inorderRight={:?}",
        //            root, postorderLeft, postorderRight, inorderLeft, inOrderRight
        //        );
        rootNode.borrow_mut().left = Solution::build_tree_internal(postorderLeft, inorderLeft);
        rootNode.borrow_mut().right = Solution::build_tree_internal(postorderRight, inOrderRight);
        //        println!("rootNode={:?}", rootNode);
        return Some(rootNode);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::share;
    use crate::share::build_tree;
    use crate::share::NULL;

    #[test]
    fn test_flattern() {
        let mut t = build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7]);
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3],),
            t
        );
    }
}
