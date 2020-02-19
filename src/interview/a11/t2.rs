/*
给定一个二叉搜索树的根结点 root, 返回树中任意两节点的差的最小值。

示例：

输入: root = [4,2,6,1,3,null,null]
输出: 1
解释:
注意，root是树结点对象(TreeNode object)，而不是数组。

给定的树 [4,2,6,1,3,null,null] 可表示为下图:

          4
        /   \
      2      6
     / \
    1   3

最小的差值是 1, 它是节点1和节点2的差值, 也是节点3和节点2的差值。
注意：

二叉树的大小范围在 2 到 100。
二叉树总是有效的，每个节点的值都是整数，且不重复
*/

/*
思路:
中序遍历,最后会得到右下角的值,
每个节点的差值都是
1. 自己减去右子树的最左子节点
2. 自己减去左子树的最右子节点
*/

use std::cmp::min;

struct Solution {}
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_value = std::i32::MAX;
        let root = root;
        Self::inorder(root, &mut min_value);
        min_value
    }
    //返回的是子树的最左左右子节点
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, min_value: &mut i32) -> (i32, i32) {
        if root.is_none() {
            panic!("impossible");
        }
        let r = root.unwrap();
        let rv = r.borrow().val;
        let mut left_most = rv;
        let mut right_most = rv;
        //如果有左子树,那么应该求左子树的最右节点
        if r.borrow().left.is_some() {
            let (l1, r1) = Self::inorder(r.borrow_mut().left.take(), min_value);
            //            println!("rv={},v={}", rv, v);
            *min_value = min(*min_value, rv - r1);
            left_most = l1;
        }
        if r.borrow().right.is_some() {
            //            println!(
            //                "rv={},right={}",
            //                rv,
            //                r.borrow().right.as_ref().unwrap().borrow().val
            //            );
            let (l2, r2) = Self::inorder(r.borrow_mut().right.take(), min_value);
            *min_value = min(*min_value, l2 - rv);
            right_most = r2;
        }
        (left_most, right_most) //如果没有右子树,它就是最大的值
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let t = build_tree_ignore_parent(&vec![4, 2, 6, 1, 3, NULL, NULL]);
        assert_eq!(Solution::min_diff_in_bst(t), 1);
        let t = build_tree_ignore_parent(&vec![
            56, 42, 77, 23, 51, 75, 90, NULL, NULL, NULL, NULL, 67, NULL, 78, 99,
        ]);
        println!("t={}", t.as_ref().unwrap().borrow().to_string());
        assert_eq!(Solution::min_diff_in_bst(t), 1);
    }
}
