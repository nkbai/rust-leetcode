/*
701. 二叉搜索树中的插入操作

给定二叉搜索树（BST）的根节点和要插入树中的值，将值插入二叉搜索树。 返回插入后二叉搜索树的根节点。 保证原始二叉搜索树中不存在新值。

注意，可能存在多种有效的插入方式，只要树在插入后仍保持为二叉搜索树即可。 你可以返回任意有效的结果。

例如,
```
给定二叉搜索树:

        4
       / \
      2   7
     / \
    1   3

和 插入的值: 5
你可以返回这个二叉搜索树:

         4
       /   \
      2     7
     / \   /
    1   3 5
或者这个树也是有效的:

         5
       /   \
      2     7
     / \
    1   3
         \
          4
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/insert-into-a-binary-search-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
按照题意,返回任意有效的bst,所以无需调整树的高度
*/
use crate::share::TreeNode;
use std::cell::{Ref, RefCell};
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode {
                left: None,
                right: None,
                val,
            })));
        }
        let r = root.clone().unwrap();
        let rv = r.borrow().val;
        let insert_node = Some(Rc::new(RefCell::new(TreeNode {
            left: None,
            right: None,
            val,
        })));
        //题目保证插入值在树中不存在
        if val < rv {
            if r.borrow().left.is_none() {
                r.borrow_mut().left = insert_node
            } else {
                Self::insert_into_bst(r.borrow().left.clone(), val);
            }
        } else {
            if r.borrow().right.is_none() {
                r.borrow_mut().right = insert_node
            } else {
                Self::insert_into_bst(r.borrow_mut().right.clone(), val);
            }
        }
        return root;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        let t = build_tree_ignore_parent(&vec![4, 2, 7, 1, 3]);
        let r = Solution::insert_into_bst(t, 5);
        assert_eq!(r, build_tree_ignore_parent(&vec![4, 2, 7, 1, 3, 5]))
    }
}
