/*
https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
105. 从前序与中序遍历序列构造二叉树
根据一棵树的前序遍历与中序遍历构造二叉树。

注意:
你可以假设树中没有重复的元素。

例如，给出

前序遍历 preorder = [3,9,20,15,7]
中序遍历 inorder = [9,3,15,20,7]
返回如下的二叉树：

    3
   / \
  9  20
    /  \
   15   7

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
首先题目中**假设树中没有重复的元素** 这句话非常关键,否则就无法构造了.
1. 在前序order中提取第一个元素作为树根
2. 然后在中序遍历中依据这个树根将数组分为前后两部分,前半部分是左子树,后半部分是右子树.
3. 前序遍历中的第二个元素就是树根的左子节点,左子节点+len(左子树)肯定是右子节点.
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
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_tree_internal(&preorder, &inorder)
    }
    fn build_tree_internal(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let (root, leftRight) = preorder.split_first().expect("must have one element");
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
        let (preOrderLeft, preorderRight) = leftRight.split_at(inorderLeft.len());
        //        println!(
        //            "root={},preOrderLeft={:?},preorderRight={:?},inorderLeft={:?},inorderRight={:?}",
        //            root, preOrderLeft, preorderRight, inorderLeft, inOrderRight
        //        );
        rootNode.borrow_mut().left = Solution::build_tree_internal(preOrderLeft, inorderLeft);
        rootNode.borrow_mut().right = Solution::build_tree_internal(preorderRight, inOrderRight);
        //        println!("rootNode={:?}", rootNode);
        return Some(rootNode);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    //    use crate::share;
    use crate::share::build_tree;
    use crate::share::NULL;

    #[test]
    fn test_flattern() {
        let mut t = build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7]);
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            t
        );
    }
}
