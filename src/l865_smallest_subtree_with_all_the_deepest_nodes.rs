/*
865. 具有所有最深结点的最小子树


给定一个根为 root 的二叉树，每个结点的深度是它到根的最短距离。

如果一个结点在整个树的任意结点之间具有最大的深度，则该结点是最深的。

一个结点的子树是该结点加上它的所有后代的集合。

返回能满足“以该结点为根的子树中包含所有最深的结点”这一条件的具有最大深度的结点。



示例：

输入：[3,5,1,6,2,0,8,null,null,7,4]
输出：[2,7,4]
解释：
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/01/sketch1.png)
我们返回值为 2 的结点，在图中用黄色标记。
在图中用蓝色标记的是树的最深的结点。
输入 "[3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]" 是对给定的树的序列化表述。
输出 "[2, 7, 4]" 是对根结点的值为 2 的子树的序列化表述。
输入和输出都具有 TreeNode 类型。


提示：

树中结点的数量介于 1 和 500 之间。
每个结点的值都是独一无二的。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/smallest-subtree-with-all-the-deepest-nodes
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
如果root左右子树深度都一样,那么root就是要找到那个节点
否则就去他深度最深的那颗子树上去找
*/
use crate::share::TreeNode;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
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
        assert_eq!(r, build_tree_ignore_parent(&vec![1, 1, 0, 1, 1, null, 1]))
    }
}
