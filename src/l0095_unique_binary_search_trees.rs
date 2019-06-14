/*
https://leetcode-cn.com/problems/unique-binary-search-trees-ii/
给定一个整数 n，生成所有由 1 ... n 为节点所组成的二叉搜索树。

示例:

输入: 3
输出:
[
  [1,null,3,2],
  [3,2,null,1],
  [3,1,null,null,2],
  [2,1,3],
  [1,null,2,null,3]
]
解释:
以上的输出对应以下 5 种不同结构的二叉搜索树：

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3
*/

/*
思路:
和第92题的思路是一样的,但是这次必须实实在在的生成这些树才可以.
只能暴力穷举
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n <= 0 {
            return Vec::new();
        }
        Solution::generate_internal(1, n + 1)
    }
    fn generate_internal(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l == r {
            return vec![None];
        } else if l == r - 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode {
                val: l,
                left: None,
                right: None,
            })))];
        } else {
            let mut v = Vec::new();
            for i in l..r {
                let lefts = Solution::generate_internal(l, i);
                let rights = Solution::generate_internal(i + 1, r);
                for lt in lefts.iter() {
                    for rt in rights.iter() {
                        v.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: lt.clone(),
                            right: rt.clone(),
                        }))));
                        //                        println!("push={:?}", v.last().unwrap());
                    }
                }
            }
            return v;
        }
    }
    //    fn appendToRight(
    //        left: Option<Rc<RefCell<TreeNode>>>,
    //        right: Option<Rc<RefCell<TreeNode>>>,
    //    ) -> Option<Rc<RefCell<TreeNode>>> {
    //        if left.is_none() {
    //            return right;
    //        }
    //        if right.is_none() {
    //            return left;
    //        }
    //        let mut rightBottom = left.clone();
    //        while rightBottom.as_ref().unwrap().borrow().right.is_some() {
    //            rightBottom = rightBottom.unwrap().borrow().right.clone()
    //        }
    //        rightBottom
    //            .as_ref()
    //            .unwrap()
    //            .borrow_mut()
    //            .right
    //            .replace(right.unwrap());
    //        left
    //    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_generate() {
        //        assert_eq!(vec![build_tree(&vec![1])], Solution::generate_trees(1));
        //        assert_eq!(
        //            vec![build_tree(&vec![1, NULL, 2]), build_tree(&vec![2, 1])],
        //            Solution::generate_trees(2)
        //        );

        let v = Solution::generate_trees(3);
        assert_eq!(5, v.len());
        for vv in v.iter() {
            println!("vv={:?}", vv);
        }
    }
}
