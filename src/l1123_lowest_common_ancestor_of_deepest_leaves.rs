/*
1123. 最深叶节点的最近公共祖先

给你一个有根节点的二叉树，找到它最深的叶节点的最近公共祖先。

回想一下：

叶节点 是二叉树中没有子节点的节点
树的根节点的 深度 为 0，如果某一节点的深度为 d，那它的子节点的深度就是 d+1
如果我们假定 A 是一组节点 S 的 最近公共祖先， S 中的每个节点都在以 A 为根节点的子树中，且 A 的深度达到此条件下可能的最大值。


示例 1：

输入：root = [1,2,3]
输出：[1,2,3]
示例 2：

输入：root = [1,2,3,4]
输出：[4]
示例 3：

输入：root = [1,2,3,4,5]
输出：[2,4,5]


提示：

给你的树中将有 1 到 1000 个节点。
树中每个节点的值都在 1 到 1000 之间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/lowest-common-ancestor-of-deepest-leaves
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
针对每个节点,
1. 如果他是叶节点,那么得到它的深度以及最近公共祖先就是他自身
2. 如果有左右节点
 无外乎两种情况
 1. 左右子树返回深度相同,那么他就是最近公共祖先,
 2. 左右子树深度不同,那么直接返回深的那个结果
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let r = Self::lca_internal(root, 0);
        return r.1;
    }
    fn lca_internal(
        root: Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
    ) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            //空,说明它的深度最低,
            return (-1, root);
        }
        depth.leading_zeros();
        let r = root.clone().unwrap();
        if r.borrow().left.is_none() && r.borrow().right.is_none() {
            return (depth, root); //自己是叶节点
        }
        let l = Self::lca_internal(r.borrow().left.clone(), depth + 1);
        let r = Self::lca_internal(r.borrow().right.clone(), depth + 1);
        if l.0 == r.0 {
            return (l.0, root);
        } else {
            if l.0 > r.0 {
                return l;
            } else {
                return r;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let t = build_tree_ignore_parent(&vec![1, 2, 3]);
        assert_eq!(Solution::lca_deepest_leaves(t).unwrap().borrow().val, 1);
        let t = build_tree_ignore_parent(&vec![1, 2, 3, 4]);
        assert_eq!(Solution::lca_deepest_leaves(t).unwrap().borrow().val, 4);
        let t = build_tree_ignore_parent(&vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::lca_deepest_leaves(t).unwrap().borrow().val, 2);
    }
}
