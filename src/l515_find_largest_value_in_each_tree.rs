/*
515. 在每个树行中找最大值
您需要在二叉树的每一行中找到最大的值。

示例：

输入:

          1
         / \
        3   2
       / \   \
      5   3   9

输出: [1, 3, 9]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-largest-value-in-each-tree-row
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
按层遍历, 每层只能逐个比较找最大值
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
            let mut max = std::i32::MIN;
            let mut vh = h.remove(0); //移除第一个
                                      //            println!("vh={:?}", vh);
            let mut vh2 = Vec::new();

            vh.iter().for_each(|t| {
                if t.borrow().val > max {
                    max = t.borrow().val;
                }
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
            r.push(max)
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
            Solution::largest_values(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
            vec![3, 20, 15]
        );
        assert_eq!(
            Solution::largest_values(build_tree(&vec![2, 1, 3])),
            vec![2, 3]
        );
        assert_eq!(
            Solution::largest_values(build_tree(&vec![
                1, 2, 3, 4, NULL, 5, 6, NULL, NULL, NULL, NULL, 7
            ])),
            vec![1, 3, 6, 7]
        );
    }
}
