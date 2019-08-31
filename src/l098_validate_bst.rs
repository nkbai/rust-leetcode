/*
98. 验证二叉搜索树
https://leetcode-cn.com/problems/validate-binary-search-tree/

给定一个二叉树，判断其是否是一个有效的二叉搜索树。

假设一个二叉搜索树具有如下特征：

节点的左子树只包含小于当前节点的数。
节点的右子树只包含大于当前节点的数。
所有左子树和右子树自身必须也是二叉搜索树。
示例 1:

输入:
    2
   / \
  1   3
输出: true
示例 2:

输入:
    5
   / \
  1   4
/ \
3   6
输出: false
解释: 输入为: [5,1,4,null,null,3,6]。
根节点的值为 5 ，但是其右子节点值为 4 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/validate-binary-search-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
二叉搜索树的特点是:左子树都小于自己,右子树都大于自己
这样就可以听过给每个子树一个最大值最小值来判断其是否是bst了
*/

use crate::share::TreeNode;
//use core::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_valid_bst_internal(root, None, None)
    }
    //因为出现这种情况[2147483647,2147483647],所以边界必须定义成option
    fn is_valid_bst_internal(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: Option<i32>,
        upper: Option<i32>,
    ) -> bool {
        if root.is_none() {
            return true;
        }
        let r = root.unwrap();
        //题目恶心人的地方就在于卡边界
        match (low, upper) {
            (None, None) => {}
            (None, Some(ref x)) => {
                if r.as_ref().borrow().val >= *x {
                    return false;
                }
            }
            (Some(ref x), None) => {
                if r.as_ref().borrow().val <= *x {
                    return false;
                }
            }
            (Some(ref l), Some(ref u)) => {
                if !(r.as_ref().borrow().val > *l && r.as_ref().borrow().val < *u) {
                    return false;
                }
            }
        }

        if !Solution::is_valid_bst_internal(
            r.as_ref().borrow().left.clone(),
            low,
            Some(r.as_ref().borrow().val),
        ) {
            return false;
        }
        if !Solution::is_valid_bst_internal(
            r.as_ref().borrow().right.clone(),
            Some(r.as_ref().borrow().val),
            upper,
        ) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_is_valid_bst() {
        let t = build_tree(&vec![2, 1, 3]);
        assert_eq!(true, Solution::is_valid_bst(t));
        let t = build_tree(&vec![5, 1, 4, NULL, NULL, 3, 6]);
        assert_eq!(false, Solution::is_valid_bst(t));
        let t = build_tree(&vec![2147483647, 2147483647]);
        assert_eq!(false, Solution::is_valid_bst(t))
    }
}
