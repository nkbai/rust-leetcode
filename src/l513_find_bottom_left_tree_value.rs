/*
513. 找树左下角的值
给定一个二叉树，在树的最后一行找到最左边的值。

示例 1:
```
输入:

    2
   / \
  1   3

输出:
1


示例 2:

输入:

        1
       / \
      2   3
     /   / \
    4   5   6
       /
      7

输出:
7
```

注意: 您可以假设树（即给定的根节点）不为 NULL。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-bottom-left-tree-value
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
逐层遍历,记录最后一层的最左侧值
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut r = Vec::new();
        let mut h = Vec::new();
        let mut bottom_left = 0;
        if root.is_none() {
            return bottom_left;
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
            let mut i = 0;
            vh.iter().for_each(|t| {
                if i == 0 {
                    bottom_left = t.borrow().val;
                }
                i += 1;
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
        bottom_left
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order_bottom() {
        assert_eq!(
            Solution::find_bottom_left_value(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            15
        );
        assert_eq!(
            Solution::find_bottom_left_value(build_tree(&vec![2, 1, 3])),
            1
        );
        assert_eq!(
            Solution::find_bottom_left_value(build_tree(&vec![
                1, 2, 3, 4, NULL, 5, 6, NULL, NULL, NULL, NULL, 7
            ])),
            7
        );
    }
}
