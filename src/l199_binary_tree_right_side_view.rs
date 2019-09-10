/*
199. 二叉树的右视图
给定一棵二叉树，想象自己站在它的右侧，按照从顶部到底部的顺序，返回从右侧所能看到的节点值。

示例:

输入: [1,2,3,null,5,null,4]
输出: [1, 3, 4]
解释:
```
   1            <---
 /   \
2     3         <---
 \     \
  5     4       <---
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-right-side-view
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。*/
/*
思路,非常比较直观
使用Vec<Vec<TreeNode>>辅助遍历
使用Vec<i32>保存结果
按层遍历思路是类似的,只不过有一点需要注意,就是只取每层最后的那个节点而已
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    //完全借鉴107的level order traversal
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
            let mut vh = h.remove(0); //移除第一个
                                      //            println!("vh={:?}", vh);
            let mut vh2 = Vec::new();
            vh.iter().for_each(|t| {
                //这里不能用map,否则会被优化掉,这里不像h会在访问的过程中被修改,所以适合使用iter而不是while
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
            r.push(vh.last().expect("must exist").borrow().val);
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
            Solution::right_side_view(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            vec![3, 20, 7]
        );
    }
}
