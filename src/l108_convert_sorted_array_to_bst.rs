/*
108. 将有序数组转换为二叉搜索树
将一个按照升序排列的有序数组，转换为一棵高度平衡二叉搜索树。

本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。

示例:

给定有序数组: [-10,-3,0,5,9],

一个可能的答案是：[0,-3,9,-10,null,5]，它可以表示下面这个高度平衡二叉搜索树：

      0
     / \
   -3   9
   /   /
 -10  5

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/convert-sorted-array-to-binary-search-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


*/

/*
思路:
递归去解,每次都是从数组中间分割,前半部分构造左子树,中间作为root,后半部分作为右子树

*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}

impl Solution {
    pub fn internal(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => {
                return None;
            }
            1 => {
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[0],
                    left: None,
                    right: None,
                })))
            }
            _ => {
                let l = nums.len();
                let (left, right) = nums.split_at(l / 2);
                let (r0, r1) = right.split_at(1);
                let root = Some(Rc::new(RefCell::new(TreeNode {
                    val: r0[0],
                    left: Solution::internal(&Vec::from(left)),
                    right: Solution::internal(&Vec::from(r1)),
                })));
                return root;
            }
        }
    }
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Solution::internal(&nums);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_num_trees() {
        let r = build_tree(&vec![0, -3, 9, -10, NULL, 5]);
        assert_eq!(Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]), r);
    }
}
