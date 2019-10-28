/*
1008. 先序遍历构造二叉树

返回与给定先序遍历 preorder 相匹配的二叉搜索树（binary search tree）的根结点。

(回想一下，二叉搜索树是二叉树的一种，其每个节点都满足以下规则，对于 node.left 的任何后代，值总 < node.val，而 node.right 的任何后代，值总 > node.val。此外，先序遍历首先显示节点的值，然后遍历 node.left，接着遍历 node.right。）



示例：

输入：[8,5,1,7,10,12]
输出：[8,5,10,1,7,null,12]
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/03/08/1266.png)


提示：

1 <= preorder.length <= 100
先序 preorder 中的值是不同的。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/construct-binary-search-tree-from-preorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
1. 根据数组的第一个元素将整个数组切分为两部分,前半部分为左子树,后半部分为右子树
2. 依次递归继续步骤1
复杂度:
1. 第一个节点比较次数为(N-1)/2
2. 树的第二层比较次数为(N-3)/2
...
因此复杂度为NlogN
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Solution::bst_from_preorder_internal(preorder.as_slice());
    }
    fn bst_from_preorder_internal(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let new_root = preorder[0];
        let r = Rc::new(RefCell::new(TreeNode {
            val: new_root,
            left: None,
            right: None,
        }));
        let pos = preorder.iter().position(|n| *n > new_root);
        match pos {
            None => {
                //找不到比new_root更大的了
                let (_, left) = preorder.split_at(1);
                r.borrow_mut().left = Solution::bst_from_preorder_internal(left);
                return Some(r);
            }
            Some(pos) => {
                //分成了左右两半部分,那么左边就是左子树,右边就是右子树
                let (first_and_left, right) = preorder.split_at(pos);
                let (_, left) = first_and_left.split_at(1);
                r.borrow_mut().left = Solution::bst_from_preorder_internal(left);
                r.borrow_mut().right = Solution::bst_from_preorder_internal(right);
                return Some(r);
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        //        let t = build_tree_ignore_parent(&vec![8, 5, 10, 1, 7, null, 12]);
        //        let r = Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]);
        //        assert_eq!(t, r);
        let t = build_tree_ignore_parent(&vec![4, 2]);
        let r = Solution::bst_from_preorder(vec![4, 2]);
        assert_eq!(t, r);
    }
}
