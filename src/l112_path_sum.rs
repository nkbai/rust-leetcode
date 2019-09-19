/*
112. 路径总和
给定一个二叉树和一个目标和，判断该树中是否存在根节点到叶子节点的路径，这条路径上所有节点值相加等于目标和。

说明: 叶子节点是指没有子节点的节点。

示例:
给定如下二叉树，以及目标和 sum = 22，

              5
             / \
            4   8
           /   / \
          11  13  4
         /  \      \
        7    2      1
返回 true, 因为存在目标和为 22 的根节点到叶子节点的路径 5->4->11->2。



来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/path-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
只能是穷举,遍历所有
因为没说存不存在负值,那么就需要把负值考虑进去
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        let mut sum = sum;
        if let Some(r) = root {
            sum -= r.borrow().val;
            if r.borrow().left.is_none() && r.borrow().right.is_none() && sum == 0 {
                return true;
            }
            if Solution::has_path_sum(r.borrow().left.clone(), sum) {
                return true;
            }
            if Solution::has_path_sum(r.borrow().right.clone(), sum) {
                return true;
            }
            return false;
        }
        return false; //叶节点,并且所求的和是0,才说明找到了有效的路径
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_has_path_sum() {
        let t = build_tree(&vec![
            5, 4, 8, 11, NULL, 13, 4, 7, 2, NULL, NULL, NULL, NULL, NULL, 1,
        ]);
        println!("t={:?}", t);
        assert_eq!(true, Solution::has_path_sum(t.clone(), 22));
        assert_eq!(false, Solution::has_path_sum(t.clone(), 23));
    }
}
