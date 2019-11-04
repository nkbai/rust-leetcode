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
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let r = root.expect("must have one node"); //至少有一个节点
        let ldepth = Solution::depth(r.borrow().left.clone());
        let rdepth = Solution::depth(r.borrow().right.clone());
        if ldepth == rdepth {
            //左右深度相同,他就是要找的了
            return Some(r);
        }
        if ldepth > rdepth {
            return Solution::subtree_with_all_deepest(r.borrow().left.clone());
        } else {
            return Solution::subtree_with_all_deepest(r.borrow().right.clone());
        }
    }
    fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
        if root.is_none() {
            return 0;
        }
        let r = root.unwrap();
        return max(
            Solution::depth(r.borrow().left.clone()),
            Solution::depth(r.borrow().right.clone()),
        ) + 1;
    }

    pub fn subtree_with_all_deepest2(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        return Solution::dfs(root, 0).1;
    }
    /*
    遍历一遍就能得到
    思路:
    针对每个结点返回两个参数 1.当前子树的最深深度 2. 当前包含所有最深结点的那颗最小子树
    初始:
    1. 每个叶节点返回的都是他的深度以及它自身
    2. 如果叶节点的父节点是左右对称的,那么返回父节点以及叶节点的深度
    3. 如果叶节点左右不对称,返回叶节点以及叶节点的深度

    */
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, d: i32) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return (-1, None);
        }
        let r = root.clone().unwrap();

        let l = Solution::dfs(r.borrow().left.clone(), d + 1);
        let r = Solution::dfs(r.borrow().right.clone(), d + 1);
        if l.0 > r.0 {
            return l;
        } else if r.0 > l.0 {
            return r;
        } else {
            //max主要是处理左右子树都为空,都是-1的情形
            return (max(l.0, d), root); //最大深度包含所有
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        let t = build_tree_ignore_parent(&vec![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]);
        let r = Solution::subtree_with_all_deepest(t.clone());
        assert_eq!(2, r.unwrap().borrow().val);
        let r = Solution::subtree_with_all_deepest2(t);
        assert_eq!(2, r.unwrap().borrow().val);
    }
}
