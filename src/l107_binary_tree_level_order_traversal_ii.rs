/*
https://leetcode-cn.com/problems/binary-tree-level-order-traversal-ii/
107. 二叉树的层次遍历 II
给定一个二叉树，返回其节点值自底向上的层次遍历。 （即按从叶子节点所在层到根节点所在的层，逐层从左向右遍历）

例如：
给定二叉树 [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7
返回其自底向上的层次遍历为：

[
  [15,7],
  [9,20],
  [3]
]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-level-order-traversal-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
这个题目和102一样,只不过是需要逆序保存结果而已
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut r = Vec::new();
        let mut h = Vec::new();
        if root.is_none() {
            return r;
        }
        //        println!("root={:?}", root);
        let mut v = Vec::new();
        v.push(root.unwrap().clone());
        h.push(v);
        while !h.is_empty() {
            let mut vr = Vec::new();
            let vh = h.remove(0); //移除第一个
                                  //            println!("vh={:?}", vh);
            let mut vh2 = Vec::new();
            vh.iter().for_each(|t| {
                //这里不能用map,否则会被优化掉,这里不像h会在访问的过程中被修改,所以适合使用iter而不是while
                vr.push(t.borrow().val);
                if let Some(l) = t.borrow().left.clone() {
                    vh2.push(l);
                }
                if let Some(r) = t.borrow().right.clone() {
                    vh2.push(r)
                }
            });
            if !vh2.is_empty() {
                h.push(vh2);
            }
            r.insert(0, vr);
        }
        r
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order_bottom() {
        assert_eq!(
            Solution::level_order_bottom(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            vec![vec![15, 7], vec![9, 20], vec![3],]
        );
    }
}
