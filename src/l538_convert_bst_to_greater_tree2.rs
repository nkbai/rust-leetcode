/*
538. 把二叉搜索树转换为累加树
给定一个二叉搜索树（Binary Search Tree），把它转换成为累加树（Greater Tree)，使得每个节点的值是原来的节点值加上所有大于它的节点值之和。

例如：
```
输入: 二叉搜索树:
              5
            /   \
           2     13

输出: 转换为累加树:
             18
            /   \
          20     13
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/convert-bst-to-greater-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
上一种方法是看错题目了,理解成普通二叉树了,
如果是bst,可以进行简化
1. 百度了一下,BST要求没有相等的节点,那么就简化了很多
2. 采用先右子树遍历,然后不断累加更新即可
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::traversal(root.clone(), 0);
        root
    }
    fn traversal(root: Option<Rc<RefCell<TreeNode>>>, mut lastv: i32) -> i32 {
        if root.is_none() {
            return lastv;
        }
        let r = root.unwrap();
        lastv = Self::traversal(r.borrow_mut().right.clone(), lastv);
        let v = r.borrow().val;
        lastv = lastv + v;
        r.borrow_mut().val = lastv;
        lastv = Self::traversal(r.borrow().left.clone(), lastv);
        return lastv;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order_bottom() {
        //        assert_eq!(
        //            Solution::convert_bst(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
        //            build_tree(&vec![])
        //        );
        assert_eq!(
            Solution::convert_bst(build_tree(&vec![2, 1, 3])),
            build_tree(&vec![5, 6, 3])
        );

        //        assert_eq!(
        //            Solution::convert_bst(build_tree(&vec![
        //                1, 2, 3, 4, NULL, 5, 6, NULL, NULL, NULL, NULL, 7
        //            ])),
        //            1
        //        );
    }
}
