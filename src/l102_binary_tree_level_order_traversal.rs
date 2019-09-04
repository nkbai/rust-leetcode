/*
102. 二叉树的层次遍历
给定一个二叉树，返回其按层次遍历的节点值。 （即逐层地，从左到右访问所有节点）。
```
例如:
给定二叉树: [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7
返回其层次遍历结果：

[
  [3],
  [9,20],
  [15,7]
]
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-level-order-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路,非常比较直观
使用Vec<Vec<TreeNode>>辅助遍历
使用Vec<Vec<i32>>保存结果
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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
            let mut vh = h.remove(0); //移除第一个
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
            r.push(vr);
        }
        r
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order() {
        assert_eq!(
            Solution::level_order(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            vec![vec![3], vec![9, 20], vec![15, 7],]
        );
    }
}
