/*

230. 二叉搜索树中第K小的元素
给定一个二叉搜索树，编写一个函数 kthSmallest 来查找其中第 k 个最小的元素。

说明：
你可以假设 k 总是有效的，1 ≤ k ≤ 二叉搜索树元素个数。

示例 1:
```
输入: root = [3,1,4,null,2], k = 1
   3
  / \
 1   4
  \
2
输出: 1
示例 2:

输入: root = [5,3,6,2,4,null,null,1], k = 3
       5
      / \
     3   6
    / \
   2   4
  /
 1
输出: 3
进阶：
如果二叉搜索树经常被修改（插入/删除操作）并且你需要频繁地查找第 k 小的值，你将如何优化 kthSmallest 函数？
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/kth-smallest-element-in-a-bst
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

/*
思路:
找到左下角开始自底向上检索,找到就结束
时间复杂度O(logN+k)
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut is_bottom_left = false;
        let mut cnt = 0;
        let mut result = 0;
        Self::internal(root, &mut |r| -> bool {
            println!("visit {}", r.borrow().val);
            if !is_bottom_left && r.borrow().left.is_none() {
                is_bottom_left = true;
            }
            if is_bottom_left {
                cnt += 1;
            }
            if cnt == k {
                result = r.borrow().val;
            }
            return cnt == k;
        });
        result
    }
    fn internal(
        root: Option<Rc<RefCell<TreeNode>>>,
        f: &mut impl FnMut(Rc<RefCell<TreeNode>>) -> bool,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        let r = root.unwrap();
        if Self::internal(r.borrow().left.clone(), f) {
            return true;
        }
        if f(r.clone()) {
            return true;
        }
        return Self::internal(r.borrow().right.clone(), f);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_count_nodes() {
        let t = build_tree(&vec![3, 1, 4, NULL, 2]);
        assert_eq!(1, Solution::kth_smallest(t.clone(), 1));
        assert_eq!(2, Solution::kth_smallest(t.clone(), 2));
        assert_eq!(3, Solution::kth_smallest(t.clone(), 3));
        assert_eq!(4, Solution::kth_smallest(t.clone(), 4));
    }
}
