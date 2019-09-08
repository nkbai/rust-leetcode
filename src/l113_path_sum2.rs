/*
113. 路径总和 II
给定一个二叉树和一个目标和，找到所有从根节点到叶子节点路径总和等于给定目标和的路径。

说明: 叶子节点是指没有子节点的节点。

示例:
给定如下二叉树，以及目标和 sum = 22，
```
              5
             / \
            4   8
           /   / \
          11  13  4
         /  \    / \
        7    2  5   1
返回:

[
   [5,4,11,2],
   [5,8,4,5]
]
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/path-sum-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
1. 使用一个vec来保存路径,真正符合的路径复制一份保存在vec中
2. 要保存所有的,所以要遍历所有才能完成
*/
use crate::share::ListNode;
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut path = Vec::new();
        let mut vv = Vec::new();
        Solution::internal(root, sum, &mut path, &mut vv);
        vv
    }
    fn internal(
        root: Option<Rc<RefCell<TreeNode>>>,
        sum: i32,
        path: &mut Vec<i32>,
        vv: &mut Vec<Vec<i32>>,
    ) {
        let mut sum = sum;
        //        println!("path={:?}", path);
        if let Some(r) = root {
            path.push(r.borrow().val);
            sum -= r.borrow().val;
            if r.borrow().left.is_none() && r.borrow().right.is_none() && sum == 0 {
                let mut v = path.clone();
                //                println!("v={:?}", v);
                vv.push(v);
                path.pop();
                return;
            }
            //走左子树
            Solution::internal(r.borrow().left.clone(), sum, path, vv);
            //走右子树
            Solution::internal(r.borrow().right.clone(), sum, path, vv);
            path.pop(); //弹出自身
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_has_path_sum() {
        /*
                [
           [5,4,11,2],
           [5,8,4,5]
        ]
                */
        let t = build_tree(&vec![
            5, 4, 8, 11, NULL, 13, 4, 7, 2, NULL, NULL, NULL, NULL, 5, 1,
        ]);
        println!("t={:?}", t);
        assert_eq!(
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]],
            Solution::path_sum(t.clone(), 22)
        );
    }
}
