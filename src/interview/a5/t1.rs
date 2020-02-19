/*
给定一个二叉树，找到最长的路径，这个路径中的每个节点具有相同值。 这条路径可以经过也可以不经过根节点。

注意：两个节点之间的路径长度由它们之间的边数表示。

示例 1:

输入:

              5
             / \
            4   5
           / \   \
          1   1   5
输出:

2
示例 2:

输入:

              1
             / \
            4   5
           / \   \
          4   4   5
输出:

2
注意: 给定的二叉树不超过10000个结点。 树的高度不超过1000。
*/
/*
思路,
每个节点返回两个值
1. 找到的最大值
2. 沿着他自己能走下去的路径的长度

*/
struct Solution {}
use crate::share::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut max, _) = Self::path(root);
        if max > 0 {
            max -= 1;
        }
        max
    }
    fn path(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let r = root.unwrap();
        let (l0, l1) = Self::path(r.borrow().left.clone());
        let (r0, r1) = Self::path(r.borrow().right.clone());
        let mut c1 = 1;
        let mut left = 1;
        let mut right = 1;
        if l1 > 0 && r.borrow().left.clone().unwrap().borrow().val == r.borrow().val {
            c1 += l1;
            left += l1;
        }
        if r1 > 0 && r.borrow().right.clone().unwrap().borrow().val == r.borrow().val {
            c1 += r1;
            right += r1;
        }
        let mut c = max(l0, r0);
        c = max(c, c1);
        (c, max(left, right))
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let v = vec![5, 4, 5, 1, 1, 5];
        let t = Solution::longest_univalue_path(build_tree_ignore_parent(&v));
        assert_eq!(t, 2);
        let v = vec![1, 4, 5, 4, 4, NULL, 5];
        let t = Solution::longest_univalue_path(build_tree_ignore_parent(&v));
        assert_eq!(t, 2);
    }
}
