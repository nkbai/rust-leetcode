/*
814. 二叉树剪枝
给定二叉树根结点 root ，此外树的每个结点的值要么是 0，要么是 1。

返回移除了所有不包含 1 的子树的原二叉树。

( 节点 X 的子树为 X 本身，以及所有 X 的后代。)

示例1:
输入: [1,null,0,0,1]
输出: [1,null,0,null,1]

解释:
只有红色节点满足条件“所有不包含 1 的子树”。
右图为返回的答案。
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_2.png)
```viz
digraph G {
    node [shape=circle]
    edge [arrowhead=vee]
    n_b1_1[label="1",color="blue"]
    n_b1_2[label="1",color="blue"]
    n_b0_1[label="0",color="blue"]
    n_b0_2[label="0",color="red"]
    n_b1_1->n_b0_1
    n_b0_1->n_b0_2
    n_b0_1->n_b1_2
}
```

示例2:
输入: [1,0,1,0,0,0,1]
输出: [1,null,1,null,1]
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_1.png)


示例3:
输入: [1,1,0,1,1,0,1,0]
输出: [1,1,0,1,1,null,1]
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/05/1028.png)


说明:

给定的二叉树最多有 100 个节点。
每个节点的值只会为 0 或 1 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-pruning
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
递归,自底向上判断要不要剪除
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::prune_internal(root.clone()) {
            root
        } else {
            None
        }
    }
    fn prune_internal(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        let r = root.unwrap();
        let mut ret = r.borrow().val == 1;
        if Self::prune_internal(r.borrow().left.clone()) {
            ret = true;
        } else {
            r.borrow_mut().left = None;
        }
        if Self::prune_internal(r.borrow().right.clone()) {
            ret = true;
        } else {
            r.borrow_mut().right = None;
        }
        return ret;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        let t = build_tree_ignore_parent(&vec![1, 1, 0, 1, 1, 0, 1, 0]);
        let r = Solution::prune_tree(t);
        println!("r={}", serialize_tree(r.clone()));
        assert_eq!(r, build_tree_ignore_parent(&vec![1, 1, 0, 1, 1, NULL, 1]))
    }
}
