/*
144. 二叉树的前序遍历
给定一个二叉树，返回它的 前序 遍历。

示例:

输入: [1,null,2,3]
   1
    \
     2
    /
   3

输出: [1,2,3]
进阶: 递归算法很简单，你可以通过迭代算法完成吗？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-preorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
非常简单，
如果要改用迭代，也很简单，就是使用vec先保存节点，再去遍历
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Solution::internal(root, &mut v);
        return v;
    }
    fn internal(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(r) = root {
            v.push(r.borrow().val);
            Solution::internal(r.borrow().left.clone(), v);
            Solution::internal(r.borrow().right.clone(), v);
        }
    }
}

#[cfg(test)]
mod test {
    #[warn(unused_imports)]
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_generate() {
        let t = build_tree(&vec![1, NULL, 2, NULL, NULL, 3]);
        assert_eq!(vec![1, 2, 3], Solution::preorder_traversal(t));
    }
}
