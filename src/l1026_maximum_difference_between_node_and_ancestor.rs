/*

1026. 节点与其祖先之间的最大差值

给定二叉树的根节点 root，找出存在于不同节点 A 和 B 之间的最大值 V，其中 V = |A.val - B.val|，且 A 是 B 的祖先。

（如果 A 的任何子节点之一为 B，或者 A 的任何子节点是 B 的祖先，那么我们认为 A 是 B 的祖先）



示例：
```viz
digraph G {
    node [shape=circle]
    edge [arrowhead=vee]
    8->3
    8->10
    3->1
    3->6
    6->4
    6->7
    10->14
    14->13
}
```


输入：[8,3,10,1,6,null,14,null,null,4,7,13]
输出：7
解释：
我们有大量的节点与其祖先的差值，其中一些如下：
|8 - 3| = 5
|3 - 7| = 4
|8 - 1| = 7
|10 - 13| = 3
在所有可能的差值中，最大值 7 由 |8 - 1| = 7 得出。


提示：

树中的节点数在 2 到 5000 之间。
每个节点的值介于 0 到 100000 之间。
在真实的面试中遇到过这道题？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-difference-between-node-and-ancestor
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
1. 用一个全局变量保存找到的最大值
2. 针对每个节点返回两个值(与所有子节点的最大值,与所有子节点的最小值)
 那么父节点在计算得到的最大绝对值肯定是和这两个值计算出来的
*/

use crate::share::TreeNode;
use std::cell::RefCell;

use std::cmp::max;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_val = 0; //最小的情况是自己和自己比,是0
        Self::max_internal(root, &mut max_val);
        return max_val;
    }
    //返回值第一个是最大值,第二个是最小值,注意不计算绝对值
    fn max_internal(root: Option<Rc<RefCell<TreeNode>>>, max_val: &mut i32) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let r = root.unwrap();
        let val = r.borrow().val;
        let left = r.borrow().left.clone();
        let mut cmax = 0;
        let mut cmin = 0;
        let mut f = |l: Rc<RefCell<TreeNode>>| {
            let mut lv = l.borrow().val;
            lv = val - lv;
            let (mut lmax, mut lmin) = Self::max_internal(Some(l.clone()), max_val);
            lmax = lv + lmax;
            lmin = lv + lmin;
            if lmax > cmax {
                cmax = lmax;
            }
            if lmin < cmin {
                cmin = lmin;
            }
        };
        if left.is_some() {
            let l = left.unwrap();
            f(l);
        }
        if r.borrow().right.is_some() {
            f(r.borrow().right.clone().unwrap());
        }
        let mut m1 = cmax;
        if m1 < 0 {
            m1 = -cmax;
        }
        let mut m2 = cmin;
        if m2 < 0 {
            m2 = -cmin;
        }
        let this_max = max(m1, m2);
        if *max_val < this_max {
            *max_val = this_max;
        }
        println!(
            "val={},cmax={},cmin={},max_val={}",
            val, cmax, cmin, max_val
        );
        return (cmax, cmin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        //        let r2 = build_tree_ignore_parent(&vec![8, 3, 10, 1, 6, null, 14, null, null, 4, 7, 13]);
        //        assert_eq!(Solution::max_ancestor_diff(r2), 7);

        let r2 = build_tree_ignore_parent(&vec![1, NULL, 2, NULL, 0, 3]);
        assert_eq!(Solution::max_ancestor_diff(r2), 3);
    }
}
