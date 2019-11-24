/*
872. 叶子相似的树

请考虑一颗二叉树上所有的叶子，这些叶子的值按从左到右的顺序排列形成一个 叶值序列 。


![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/16/tree.png)
举个例子，如上图所示，给定一颗叶值序列为 (6, 7, 4, 9, 8) 的树。

如果有两颗二叉树的叶值序列是相同，那么我们就认为它们是 叶相似 的。

如果给定的两个头结点分别为 root1 和 root2 的树是叶相似的，则返回 true；否则返回 false 。



提示：

给定的两颗树可能会有 1 到 100 个结点。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/leaf-similar-trees
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
思路很简单
1. 分别遍历每棵树,
2. 把叶子存成一个数组
3. 比较两个数组是否相同
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        Self::leafs(root1, &mut v1);
        Self::leafs(root2, &mut v2);
        if v1.len() != v2.len() {
            return false;
        }
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                return false;
            }
        }
        return true;
    }
    pub fn leafs(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap();
        if r.borrow().left.is_none() && r.borrow().right.is_none() {
            v.push(r.borrow().val);
        } else {
            Self::leafs(r.borrow().left.clone(), v);
            Self::leafs(r.borrow().right.clone(), v);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_construct_tree() {
        let t1 = build_tree_ignore_parent(&vec![3, 5, 1, 6, 2, 9, 8, NULL, NULL, 7, 4]);
        let t2 = build_tree_ignore_parent(&vec![4, 9, 1, 6, 3, 9, 8, NULL, NULL, 7, 4]);
        assert_eq!(Solution::leaf_similar(t1.clone(), t2.clone()), true);
        let t3 = build_tree_ignore_parent(&vec![4, 9, 1, 6, 3, 9, 8, NULL, NULL, 7, 4, 9, 11]);
        assert_eq!(Solution::leaf_similar(t1.clone(), t3.clone()), false)
    }
}
