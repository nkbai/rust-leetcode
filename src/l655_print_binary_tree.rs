/*
655. 输出二叉树
在一个 m*n 的二维字符串数组中输出二叉树，并遵守以下规则：

行数 m 应当等于给定二叉树的高度。
列数 n 应当总是奇数。
根节点的值（以字符串格式给出）应当放在可放置的第一行正中间。根节点所在的行与列会将剩余空间划分为两部分（左下部分和右下部分）。你应该将左子树输出在左下部分，右子树输出在右下部分。左下和右下部分应当有相同的大小。即使一个子树为空而另一个非空，你不需要为空的子树输出任何东西，但仍需要为另一个子树留出足够的空间。然而，如果两个子树都为空则不需要为它们留出任何空间。
每个未使用的空间应包含一个空的字符串""。
使用相同的规则输出子树。
示例 1:

输入:
     1
    /
   2
输出:
[["", "1", ""],
 ["2", "", ""]]
示例 2:

输入:
     1
    / \
   2   3
    \
     4
输出:
[["", "", "", "1", "", "", ""],
 ["", "2", "", "", "", "3", ""],
 ["", "", "4", "", "", "", ""]]
示例 3:

输入:
      1
     / \
    2   5
   /
  3
 /
4
输出:
[["",  "",  "", "",  "", "", "", "1", "",  "",  "",  "",  "", "", ""]
 ["",  "",  "", "2", "", "", "", "",  "",  "",  "",  "5", "", "", ""]
 ["",  "3", "", "",  "", "", "", "",  "",  "",  "",  "",  "", "", ""]
 ["4", "",  "", "",  "", "", "", "",  "",  "",  "",  "",  "", "", ""]]
注意: 二叉树的高度在范围 [1, 10] 中。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/print-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
注意: 二叉树的高度在范围 [1, 10] 中。
*/
/*
思路:
 先求出树的高度,那么每层宽度就确定了,
 1. 假设树高为3,那么root节点打印出来就应该是左边2^(3-1)-1个空,自身,右边2^(3-1)-1个空
 2. 递归向下,左边打印出来的就是左侧整棵子树的样子,右边打印出来就是右侧整颗子树的样子,
 3. 将左右合并
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let height = Self::height(root.clone());
        //        println!("height is {}", height);
        Self::print_tree_internal(root, height)
    }
    fn print_tree_internal(root: Option<Rc<RefCell<TreeNode>>>, height: i32) -> Vec<Vec<String>> {
        let val = if root.is_none() {
            String::from("")
        } else {
            format!("{}", root.clone().unwrap().borrow().val)
        };

        if height == 1 {
            return vec![vec![String::from(val)]];
        }
        /*
        先凑出我这一层
        */
        let mut result = Vec::new();
        let space_count = (1 << (height - 1)) - 1;
        //        println!("height={},space_count={}", height, space_count);
        let mut this_level = Vec::new();
        for _ in 0..space_count {
            this_level.push(String::from(""))
        }
        this_level.push(val);
        for _ in 0..space_count {
            this_level.push(String::from(""))
        }
        result.push(this_level);
        let (leftNode, rightNode) = if root.is_none() {
            (None, None)
        } else {
            (
                root.clone().unwrap().borrow().left.clone(),
                root.clone().unwrap().borrow().right.clone(),
            )
        };
        let left = Self::print_tree_internal(leftNode, height - 1);
        let right = Self::print_tree_internal(rightNode, height - 1);
        //将左右拼接起来,保证宽度
        for i in 0..left.len() {
            let mut v = Vec::new();
            v.extend_from_slice(&left[i]);
            v.push(String::from(""));
            v.extend_from_slice(&right[i]);
            result.push(v);
        }
        result
    }
    fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let r = root.unwrap();
        let left = Self::height(r.borrow().left.clone());
        let height = Self::height(r.borrow().right.clone());
        return max(left, height) + 1;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        let t = build_tree_ignore_parent(&vec![1, 2, 3, NULL, 4]);
        assert_eq!(
            Solution::print_tree(t),
            vec![
                vec!["", "", "", "1", "", "", ""],
                vec!["", "2", "", "", "", "3", ""],
                vec!["", "", "4", "", "", "", ""]
            ]
        )
    }
}
