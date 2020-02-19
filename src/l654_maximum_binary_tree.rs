/*
654. 最大二叉树

给定一个不含重复元素的整数数组。一个以此数组构建的最大二叉树定义如下：

二叉树的根是数组中的最大元素。
左子树是通过数组中最大值左边部分构造出的最大二叉树。
右子树是通过数组中最大值右边部分构造出的最大二叉树。
通过给定的数组构建最大二叉树，并且输出这个树的根节点。



示例 ：

输入：[3,2,1,6,0,5]
输出：返回下面这棵树的根节点：

      6
    /   \
   3     5
    \    /
     2  0
       \
        1


提示：

给定的数组的大小在 [1, 1000] 之间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
就是一个检索分类的过程
1. 找到现有最大值,这就是当前节点
2. 左边如果有,左边最大值就是左子树的根节点
2. 右边如果有,右边最大值就是右子树的根节点
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::internal(&nums)
    }
    fn internal(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        let mut max_pos: i32 = -1;
        let mut max_value = std::i32::MIN; //如果
        nums.iter().enumerate().for_each(|v| {
            if *v.1 > max_value {
                max_pos = v.0 as i32;
                max_value = *v.1;
            }
        });
        if max_pos == -1 {
            max_pos = 0; //全是std::i32::MIN?
        }
        let (left, right) = nums.split_at(max_pos as usize);
        let (r1, r2) = right.split_at(1); //右边肯定会有一个数值
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: r1[0],
            left: Self::construct_maximum_binary_tree(Vec::from(left)),
            right: Self::construct_maximum_binary_tree(Vec::from(r2)),
        })));
        return root;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        let t = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
        assert_eq!(
            t,
            build_tree_ignore_parent(&vec![6, 3, 5, NULL, 2, 0, NULL, NULL, 1])
        )
    }
}
