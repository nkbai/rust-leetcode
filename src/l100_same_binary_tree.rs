/*
100. 相同的树

给定两个二叉树，编写一个函数来检验它们是否相同。

如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。
```
示例 1:

输入:       1         1
          / \       / \
         2   3     2   3

        [1,2,3],   [1,2,3]

输出: true
示例 2:

输入:      1          1
          /           \
         2             2

        [1,2],     [1,null,2]

输出: false
示例 3:

输入:       1         1
          / \       / \
         2   1     1   2

        [1,2,1],   [1,1,2]

输出: false
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/same-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

/*
思路:
很简单,返回false不继续,否则继续递归
*/
use crate::share::TreeNode;
use rand::distributions::uniform::SampleBorrow;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                if p.borrow().val != q.borrow().val {
                    return false;
                } else {
                    if !Solution::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone()) {
                        return false;
                    }
                    if !Solution::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone()) {
                        return false;
                    }
                    return true;
                }
            }
            (None, None) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_num_trees() {
        let mut r = build_tree(&vec![1, 3, NULL, NULL, 2]);
        let mut r2 = build_tree(&vec![1, 3, NULL, NULL, 2]);
        assert_eq!(Solution::is_same_tree(r, r2), true);
        let mut r = build_tree(&vec![1, 3, NULL, NULL, 2]);
        let mut r3 = build_tree(&vec![3, 1, 4, NULL, NULL, 2]);
        assert_eq!(Solution::is_same_tree(r, r3), false);
    }
}
