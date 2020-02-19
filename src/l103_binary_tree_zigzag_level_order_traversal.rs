/*
103. 二叉树的锯齿形层次遍历

给定一个二叉树，返回其节点值的锯齿形层次遍历。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。
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
  [20,9],
  [15,7]
]
```

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut r = Vec::new();
        let mut h = Vec::new();
        if root.is_none() {
            return r;
        }
        //        println!("root={:?}", root);
        let mut v = Vec::new();
        v.push(root.unwrap().clone());
        h.push(v);
        let mut is_reverse = false;
        while !h.is_empty() {
            let mut vr = Vec::new();
            let vh = h.remove(0); //移除第一个
                                  //            println!("vh={:?}", vh);
            let mut vh2 = Vec::new();
            vh.iter().rev().for_each(|t| {
                //这里不能用map,否则会被优化掉,这里不像h会在访问的过程中被修改,所以适合使用iter而不是while
                vr.push(t.borrow().val);
                if is_reverse {
                    if let Some(r) = t.borrow().right.clone() {
                        vh2.push(r)
                    }
                    if let Some(l) = t.borrow().left.clone() {
                        vh2.push(l);
                    }
                } else {
                    if let Some(l) = t.borrow().left.clone() {
                        vh2.push(l);
                    }
                    if let Some(r) = t.borrow().right.clone() {
                        vh2.push(r)
                    }
                }
            });
            if !vh2.is_empty() {
                h.push(vh2);
            }
            r.push(vr);
            is_reverse = !is_reverse;
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
            Solution::zigzag_level_order(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            vec![vec![3], vec![20, 9], vec![15, 7],]
        );
        assert_eq!(
            Solution::zigzag_level_order(build_tree(&vec![
                3, 9, 20, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23
            ])),
            vec![
                vec![3],
                vec![20, 9],
                vec![11, 13, 14, 15],
                vec![23, 22, 21, 20, 19, 18, 17, 16],
            ]
        );
    }
}
