/*
623. 在二叉树中增加一行
给定一个二叉树，根节点为第1层，深度为 1。在其第 d 层追加一行值为 v 的节点。

添加规则：给定一个深度值 d （正整数），针对深度为 d-1 层的每一非空节点 N，为 N 创建两个值为 v 的左子树和右子树。

将 N 原先的左子树，连接为新节点 v 的左子树；将 N 原先的右子树，连接为新节点 v 的右子树。

如果 d 的值为 1，深度 d - 1 不存在，则创建一个新的根节点 v，原先的整棵树将作为 v 的左子树。

示例 1:

输入:
二叉树如下所示:
       4
     /   \
    2     6
   / \   /
  3   1 5

v = 1

d = 2

输出:
       4
      / \
     1   1
    /     \
   2       6
  / \     /
 3   1   5

示例 2:

输入:
二叉树如下所示:
      4
     /
    2
   / \
  3   1

v = 1

d = 3

输出:
      4
     /
    2
   / \
  1   1
 /     \
3       1
注意:

输入的深度值 d 的范围是：[1，二叉树最大深度 + 1]。
输入的二叉树至少有一个节点。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/add-one-row-to-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
按层遍历,先找到d-1层,
考虑特殊情况:
1. 就是d-1等于0的情况,那么就是新增根节点.
2. 其他情况逐个添加即可.
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        /*
        先考虑d等于1这种特殊情况
        */
        if d == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                left: root,
                right: None,
                val: v,
            })));
        }
        let r = root.unwrap();
        //考虑其他,上一层肯定是有效的
        let mut currentLayer = Vec::new();
        currentLayer.push(r.clone());
        let mut depth = 1;
        while !currentLayer.is_empty() {
            if depth == d - 1 {
                //题目中严格定义了d的范围,所以不用担心层数无效的问题
                //找到了要操作的那一层
                currentLayer.iter().for_each(|t| {
                    let left = t.borrow_mut().left.take();
                    let right = t.borrow_mut().right.take();
                    t.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                        left,
                        right: None,
                        val: v,
                    })));
                    t.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                        right,
                        left: None,
                        val: v,
                    })));
                })
            } else {
                let mut nextLayer = Vec::new();
                currentLayer.iter().for_each(|t| {
                    //这里不能用map,否则会被优化掉,这里不像h会在访问的过程中被修改,所以适合使用iter而不是while
                    if let Some(l) = t.borrow().left.clone() {
                        nextLayer.push(l);
                    }
                    if let Some(r) = t.borrow().right.clone() {
                        nextLayer.push(r);
                    }
                });
                currentLayer = nextLayer;
            }
            depth += 1;
        }
        Some(r)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_is_sub_tree() {
        assert_eq!(
            Solution::add_one_row(build_tree_ignore_parent(&vec![3, 4, 5, 1, 2]), 1, 1),
            build_tree_ignore_parent(&vec![1, 3, NULL, 4, 5, 1, 2])
        );

        assert_eq!(
            Solution::add_one_row(build_tree_ignore_parent(&vec![3, 4, 5, 1, 2]), 1, 2),
            build_tree_ignore_parent(&vec![3, 1, 1, 4, NULL, NULL, 5, 1, 2])
        );
        assert_eq!(
            Solution::add_one_row(build_tree_ignore_parent(&vec![3, 4, 5, 1, 2]), 1, 3),
            build_tree_ignore_parent(&vec![3, 4, 5, 1, 1, 1, 1, 1, NULL, NULL, 2])
        );

        assert_eq!(
            Solution::add_one_row(build_tree_ignore_parent(&vec![3, 4, 5, 1, 2]), 1, 4),
            build_tree_ignore_parent(&vec![3, 4, 5, 1, 2, NULL, NULL, 1, 1, 1, 1])
        );
    }
}
