/*

226. 翻转二叉树
翻转一棵二叉树。

示例：

输入：
```
     4
   /   \
  2     7
 / \   / \
1   3 6   9
输出：

     4
   /   \
  7     2
 / \   / \
9   6 3   1
```
备注:
这个问题是受到 Max Howell 的 原问题 启发的 ：

谷歌：我们90％的工程师使用您编写的软件(Homebrew)，但是您却无法在面试时在白板上写出翻转二叉树这道题，这太糟糕了。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/invert-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
看清楚题目:
整棵树的镜像,递归非常简单
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root = root.unwrap();
        let l = Self::invert_tree(root.borrow_mut().left.take());
        let r = Self::invert_tree(root.borrow_mut().right.take());
        root.borrow_mut().left = r;
        root.borrow_mut().right = l;
        return Some(root);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order() {
        assert_eq!(
            Solution::invert_tree(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            build_tree(&vec![3, 20, 9, 7, 15])
        );
    }
}
