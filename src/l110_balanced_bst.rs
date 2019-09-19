/*
 110. 平衡二叉树

给定一个二叉树，判断它是否是高度平衡的二叉树。

本题中，一棵高度平衡二叉树定义为：

一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过1。
```
示例 1:

给定二叉树 [3,9,20,null,null,15,7]

    3
   / \
  9  20
    /  \
   15   7
返回 true 。

示例 2:

给定二叉树 [1,2,2,3,3,null,null,4,4]

       1
      / \
     2   2
    / \
   3   3
  / \
 4   4
返回 false 。
```

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/balanced-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
递归 判断左右两颗子树的高度
如果碰到某颗子树不是,立即返回,否则继续尝试
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (_, b) = Solution::internal(root);
        return b;
    }
    fn internal(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if let Some(r) = root {
            let left = r.borrow().left.clone();
            let right = r.borrow().right.clone();
            let lh = Solution::internal(left);
            let rh = Solution::internal(right);
            if lh.1 == false || rh.1 == false {
                return (0, false);
            }
            if lh.0 - rh.0 > 1 || rh.0 - lh.0 > 1 {
                return (0, false);
            }
            let h = max(lh.0, rh.0);
            return (h + 1, true);
        }
        return (0, true);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_sorted_list_to_bst() {
        let t = build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7]);
        assert_eq!(Solution::is_balanced(t), true);

        let t = build_tree(&vec![1, 2, 2, 3, 3, NULL, NULL, 4, 4]);
        assert_eq!(Solution::is_balanced(t), false);
        //[1,2,2,3,3,null,null,4,4] false
    }
}
