/*
337. 打家劫舍 III
在上次打劫完一条街道之后和一圈房屋后，小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为“根”。 除了“根”之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果两个直接相连的房子在同一天晚上被打劫，房屋将自动报警。

计算在不触动警报的情况下，小偷一晚能够盗取的最高金额。

示例 1:
```
输入: [3,2,3,null,3,null,1]

     3
    / \
   2   3
    \   \
     3   1

输出: 7
解释: 小偷一晚能够盗取的最高金额 = 3 + 3 + 1 = 7.
示例 2:

输入: [3,4,5,1,3,null,1]

3
    / \
   4   5
  / \   \
 1   3   1
```
输出: 9
解释: 小偷一晚能够盗取的最高金额 = 4 + 5 = 9.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/house-robber-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
遍历每个节点,都返回两个值:
1. 经过自身的可能的最高金额
2. 不经过自身的可能的最高金额
自底向上开始遍历,
对于当前节点:
1. 如果没有孩子节点,那么经过自身的最高金额就是自身0,不经过的同样也是0
2. 如果有左右孩子节点,那么经过自身的最高金额就是自身+ 不经过左右子节点之和,
 不经过自身的最高金额稍微复杂一点:
  因为左右子节点的返回四个值可以自由组合,因此都选最大的即可
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (rc, ri) = Self::internal(root);
        max(rc, ri)
    }
    /*
    返回值:
    第一个: 经过自身的最大金额
    第二个:不经过自身的最大金额
    */
    fn internal(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let r = root.unwrap();
        let (lc, li) = Self::internal(r.borrow_mut().left.take());
        let (rc, ri) = Self::internal(r.borrow_mut().right.take());
        let v = r.borrow().val;
        //么经过自身的最高金额就是自身+不经过左右子节点之和,
        let cc = v + li + ri;
        //不经过自身 可能性比较多,选经不经过左右子节点的都是可以的
        let ci = max(lc, li) + max(rc, ri);
        println!("v={},cc={},ci={}", v, cc, ci);
        return (cc, ci);
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
        let t = build_tree(&vec![3, 2, 3, NULL, 3, NULL, 1]);
        assert_eq!(7, Solution::rob(t));
        let t = build_tree(&vec![4, 1, NULL, 2, NULL, NULL, NULL, 3]);
        assert_eq!(7, Solution::rob(t));
        let t = build_tree(&vec![2, 1, 3, NULL, 4]);
        assert_eq!(7, Solution::rob(t));
    }
}
