/*
111. 二叉树的最小深度

给定一个二叉树，找出其最小深度。

最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

说明: 叶子节点是指没有子节点的节点。

示例:

给定二叉树 [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7
返回它的最小深度  2.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-depth-of-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。*/

/*
思路:
何求最大深度一样的,只不过返回矮的那颗而已
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let hasLeft = r.borrow().left.is_some();
            let hasRight = r.borrow().right.is_some();
            let mut left = Solution::min_depth(r.borrow_mut().left.take());
            let mut right = Solution::min_depth(r.borrow_mut().right.take());
            if hasLeft && hasRight {
                if left > right {
                    return right + 1;
                } else {
                    return left + 1;
                }
            } else if hasLeft {
                return left + 1;
            } else if hasRight {
                return right + 1;
            } else {
                return 1;
            }
        }
        return 0;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order() {
        assert_eq!(
            Solution::min_depth(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            2
        );
        assert_eq!(Solution::min_depth(build_tree(&vec![1, 2])), 2);
    }
}
