/*
958. 二叉树的完全性检验

给定一个二叉树，确定它是否是一个完全二叉树。

百度百科中对完全二叉树的定义如下：

若设二叉树的深度为 h，除第 h 层外，其它各层 (1～h-1) 的结点数都达到最大个数，第 h 层所有的结点都连续集中在最左边，这就是完全二叉树。（注：第 h 层可能包含 1~ 2h 个节点。）



示例 1：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/15/complete-binary-tree-1.png)

输入：[1,2,3,4,5,6]
输出：true
解释：最后一层前的每一层都是满的（即，结点值为 {1} 和 {2,3} 的两层），且最后一层中的所有结点（{4,5,6}）都尽可能地向左。
示例 2：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/15/complete-binary-tree-2.png)

输入：[1,2,3,4,5,null,7]
输出：false
解释：值为 7 的结点没有尽可能靠向左侧。


提示：

树中将会有 1 到 100 个结点。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/check-completeness-of-a-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
完全二叉树的只能逐层判断
1.除最后一层外必须其他每层必须是满的
2. 最后一层所有子节点必须尽可能靠左
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut current_level = Vec::new();
        //        let r=root.expect("at least one node"); //题目中说了至少有一个节点
        current_level.push(root);
        loop {
            let mut next_level = Vec::new();
            let mut has_none = false;
            let res = current_level.iter().try_for_each(|n| {
                match n {
                    None => has_none = true,
                    Some(n) => {
                        if has_none {
                            return Err(false); //不允许出现空,然后再出现非空
                        }
                        next_level.push(n.borrow().left.clone());
                        next_level.push(n.borrow().right.clone());
                    }
                }
                return Ok(());
            });
            if res.is_err() {
                return false;
            }
            if has_none {
                //最后一层应该没有任何子节点
                let res = next_level.iter().try_for_each(|n| {
                    if n.is_some() {
                        return Err(false);
                    }
                    return Ok(());
                });
                if res.is_err() {
                    return false;
                }
                return true;
            } else {
                current_level = next_level;
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_sorted_list_to_bst() {
        let t = build_tree_ignore_parent(&vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(Solution::is_complete_tree(t), true);
        let t = build_tree_ignore_parent(&vec![1, 2, 3, 4, 5, NULL, 7]);
        assert_eq!(Solution::is_complete_tree(t), false);
    }
}
