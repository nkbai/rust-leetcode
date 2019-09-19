/*
257. 二叉树的所有路径
给定一个二叉树，返回所有从根节点到叶子节点的路径。

说明: 叶子节点是指没有子节点的节点。

示例:

输入:

   1
 /   \
2     3
 \
  5

输出: ["1->2->5", "1->3"]

解释: 所有根节点到叶子节点的路径为: 1->2->5, 1->3
在真实的面试中遇到过这道题？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-paths
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
记得每个节点路径都是clone过去的，可以任意修改
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut v = Vec::new();
        let path = String::new();
        Self::internal(root, path, &mut |n, p| {
            if p.len() == 0 {
                *p += n.borrow().val.to_string().as_str();
            } else {
                *p += "->";
                *p += n.borrow().val.to_string().as_str();
            }
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                v.push(p.clone());
            }
        });
        return v;
    }
    fn internal(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut path: String,
        f: &mut impl FnMut(Rc<RefCell<TreeNode>>, &mut String), //闭包类型不能带参数名字，
    ) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap();
        f(r.clone(), &mut path);
        Self::internal(r.borrow_mut().left.take(), path.clone(), f);
        Self::internal(r.borrow_mut().right.take(), path, f);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    #[test]
    fn test_path() {
        let t = build_tree(&vec![1, 2, 3, 4, 5]);
        assert_eq!(
            vec!["1->2->4", "1->2->5", "1->3",],
            Solution::binary_tree_paths(t)
        );
    }
}
