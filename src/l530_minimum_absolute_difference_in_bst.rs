/*
530. 二叉搜索树的最小绝对差
给定一个所有节点为非负值的二叉搜索树，求树中任意两节点的差的绝对值的最小值。

示例 :

输入:
```
   1
    \
     3
    /
   2
```
输出:
1

解释:
最小绝对差为1，其中 2 和 1 的差的绝对值为 1（或者 2 和 3）。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-absolute-difference-in-bst
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
因为此题目是简单类型,那就按照最粗暴的方式来处理
遍历所有,然后排序,然后逐个挑选
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = Vec::new();
        Self::traversal(root, &mut v);
        v.sort();
        let mut a = std::i32::MAX;
        for i in 1..v.len() {
            let mut c = v[i] - v[i - 1];
            if c < 0 {
                c = -c;
            }
            a = min(c, a);
        }
        a
    }
    fn traversal(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap();
        v.push(r.borrow().val);
        Self::traversal(r.borrow().left.clone(), v);
        Self::traversal(r.borrow_mut().right.clone(), v);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order_bottom() {
        assert_eq!(
            Solution::get_minimum_difference(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            2
        );
        assert_eq!(
            Solution::get_minimum_difference(build_tree(&vec![2, 1, 3])),
            1
        );
        assert_eq!(
            Solution::get_minimum_difference(build_tree(&vec![
                1, 2, 3, 4, NULL, 5, 6, NULL, NULL, NULL, NULL, 7
            ])),
            1
        );
    }
}
