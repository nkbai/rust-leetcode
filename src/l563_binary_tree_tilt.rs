/*
563. 二叉树的坡度
给定一个二叉树，计算整个树的坡度。

一个树的节点的坡度定义即为，该节点左子树的结点之和和右子树结点之和的差的绝对值。空结点的的坡度是0。

整个树的坡度就是其所有节点的坡度之和。

示例:

输入:
         1
       /   \
      2     3
输出: 1
解释:
结点的坡度 2 : 0
结点的坡度 3 : 0
结点的坡度 1 : |2-3| = 1
树的坡度 : 0 + 0 + 1 = 1
注意:

任何子树的结点的和不会超过32位整数的范围。
坡度的值不会超过32位整数的范围。
在真实的面试中遇到过这道题？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-tilt
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
看懂题目,剩下就简单了.

*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::find(root, &mut |c| {
            sum += c;
        });
        return sum;
    }
    fn find(root: Option<Rc<RefCell<TreeNode>>>, f: &mut impl FnMut(i32)) -> i32 {
        if root.is_none() {
            return 0;
        }
        let r = root.unwrap();

        let left = if r.borrow().left.is_none() {
            0
        } else {
            let l = r.borrow().left.clone().unwrap();
            let l1 = Self::find(Some(l.clone()), f);
            let l2 = l.borrow().val;
            l1 + l2
        };
        let right = if r.borrow().right.is_none() {
            0
        } else {
            let l = r.borrow().right.clone().unwrap();
            let l1 = Self::find(Some(l.clone()), f);
            let l2 = l.borrow().val;
            l1 + l2
        };

        let c = if left - right > 0 {
            left - right
        } else {
            right - left
        };
        f(c);
        left + right
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order_bottom() {
        assert_eq!(Solution::find_tilt(build_tree(&vec![1, 2, 3])), 1);
    }
}

//怎么才能写的更优雅呢
