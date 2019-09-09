/*
129. 求根到叶子节点数字之和

给定一个二叉树，它的每个结点都存放一个 0-9 的数字，每条从根到叶子节点的路径都代表一个数字。

例如，从根到叶子节点路径 1->2->3 代表数字 123。

计算从根到叶子节点生成的所有数字之和。

说明: 叶子节点是指没有子节点的节点。

示例 1:

输入: [1,2,3]
    1
   / \
  2   3
输出: 25
解释:
从根到叶子节点路径 1->2 代表数字 12.
从根到叶子节点路径 1->3 代表数字 13.
因此，数字总和 = 12 + 13 = 25.
示例 2:

输入: [4,9,0,5,1]
    4
   / \
  9   0
/ \
5   1
输出: 1026
解释:
从根到叶子节点路径 4->9->5 代表数字 495.
从根到叶子节点路径 4->9->1 代表数字 491.
从根到叶子节点路径 4->0 代表数字 40.
因此，数字总和 = 495 + 491 + 40 = 1026.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sum-root-to-leaf-numbers
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
简单起见，不用vec保存路径，只需要保存到当前节点路径的和即可，
如果要回溯，简单减去自身，然后除以十即可。
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Solution::internal(root, 0, &mut sum);
        return sum;
    }
    fn internal(root: Option<Rc<RefCell<TreeNode>>>, mut path_sum: i32, sum: &mut i32) {
        if let Some(r) = root {
            path_sum = path_sum * 10 + r.borrow().val;
            if r.borrow().left.is_none() && r.borrow().right.is_none() {
                *sum += path_sum; //找到了一条路径
            }
            //简单起见不做区分，尝试左右子节点
            Solution::internal(r.borrow().left.clone(), path_sum, sum);
            Solution::internal(r.borrow().right.clone(), path_sum, sum);
        }
    }
}

#[cfg(test)]
mod test {
    #[warn(unused_imports)]
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_generate() {
        let t = build_tree(&vec![4, 9, 0, 5, 1]);
        assert_eq!(1026, Solution::sum_numbers(t));
        let t = build_tree(&vec![1, 2, 3]);
        assert_eq!(25, Solution::sum_numbers(t));
    }
}
