/*
572. 另一个树的子树

给定两个非空二叉树 s 和 t，检验 s 中是否包含和 t 具有相同结构和节点值的子树。s 的一个子树包括 s 的一个节点和这个节点的所有子孙。s 也可以看做它自身的一棵子树。
```
示例 1:
给定的树 s:

     3
    / \
   4   5
  / \
 1   2
给定的树 t：

   4
  / \
 1   2
返回 true，因为 t 与 s 的一个子树拥有相同的结构和节点值。

示例 2:
给定的树 s：

     3
    / \
   4   5
  / \
 1   2
    /
   0
给定的树 t：

   4
  / \
 1   2
返回 false。
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/subtree-of-another-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
简单类型,暴力穷举即可
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::internal(s, t, false);
    }
    fn internal(
        s: Option<Rc<RefCell<TreeNode>>>,
        t: Option<Rc<RefCell<TreeNode>>>,
        is_part: bool,
    ) -> bool {
        /*
        按照题意,s中间的某一段包含t也是不行的
        */
        if t.is_none() && s.is_none() {
            return true;
        } else if t.is_none() {
            return false;
        } else if s.is_none() {
            return false;
        }
        let s = s.unwrap();
        let t = t.unwrap();
        //如果s和t刚好是子树的一部分,那就匹配了
        if s.borrow().val == t.borrow().val {
            let left = Self::internal(s.borrow().left.clone(), t.borrow().left.clone(), true);
            let right = Self::internal(s.borrow().right.clone(), t.borrow().right.clone(), true);
            if left && right {
                return true;
            } else if is_part {
                //如果已经比较了t的一部分,只要不匹配,就不应该继续下去了
                return false;
            }
        } else if is_part {
            return false;
        }
        //如果没匹配,继续穷举,这与s.val和t.val是否相等没关系,不是互斥关系
        if Self::internal(s.borrow().left.clone(), Some(t.clone()), false) {
            return true;
        } else if Self::internal(s.borrow().right.clone(), Some(t.clone()), false) {
            return true;
        }
        return false;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_is_sub_tree() {
        assert_eq!(
            Solution::is_subtree(
                build_tree_ignore_parent(&vec![3, 4, 5, 1, 2]),
                build_tree_ignore_parent(&vec![4, 1, 2])
            ),
            true
        );
        assert_eq!(
            Solution::is_subtree(
                build_tree_ignore_parent(&vec![3, 4, 5, 1, 2, null, null, null, null, 0]),
                build_tree_ignore_parent(&vec![4, 1, 2])
            ),
            false
        );
        assert_eq!(
            Solution::is_subtree(
                build_tree_ignore_parent(&vec![3, 4, 5, 1, null, 2]),
                build_tree_ignore_parent(&vec![3, 1, 2])
            ),
            false
        );
    }
}
