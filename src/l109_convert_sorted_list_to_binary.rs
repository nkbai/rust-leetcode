/*
https://leetcode-cn.com/problems/convert-sorted-list-to-binary-search-tree/
给定一个单链表，其中的元素按升序排序，将其转换为高度平衡的二叉搜索树。

本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。

示例:

给定的有序链表： [-10, -3, 0, 5, 9],

一个可能的答案是：[0, -3, 9, -10, null, 5], 它可以表示下面这个高度平衡二叉搜索树：

      0
     / \
   -3   9
   /   /
 -10  5
*/

/*
思路:
这是一个递归求解问题:
首先把链表从中间切开,那么这时候主题是一个相对平衡的二叉树
左边,右边分别像第一步一样递归求解即可.
*/
use crate::share::ListNode;
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let v = Solution::list_to_vec(head);
        let nodes = v.as_slice();
        Solution::build_tree(nodes)
    }
    fn build_tree(nodes: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nodes.len() == 0 {
            return None;
        }
        let left = nodes.len() / 2;
        let right = nodes.len() / 2 + 1;
        let mut left_node: Option<Rc<RefCell<TreeNode>>> = None;
        let mut right_node = None;
        if left > 0 {
            left_node = Solution::build_tree(&nodes[0..left]);
        }
        if right <= nodes.len() - 1 {
            right_node = Solution::build_tree(&nodes[right..nodes.len()])
        }
        Some(Rc::new(RefCell::new(TreeNode {
            val: nodes[nodes.len() / 2],
            left: left_node,
            right: right_node,
        })))
    }
    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut rhead = head.as_ref();
        let mut v = Vec::new();
        while let Some(h) = rhead {
            v.push(h.val);
            rhead = h.next.as_ref();
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_list_node;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_sorted_list_to_bst() {
        let l1 = build_list_node(&vec![-10, -3, 0, 5, 9]);
        let t = build_tree(&vec![0, -3, 9, -10, NULL, 5]);
        //        println!("t={:?}", t);
        //        println!("sorted={:?}", Solution::sorted_list_to_bst(l1));
        assert_eq!(Solution::sorted_list_to_bst(l1), t);

        let l1 = build_list_node(&vec![3, 5, 8]);
        let t = build_tree(&vec![5, 3, 8]);
        //        println!("t={:?}", t);
        //        println!("sorted={:?}", Solution::sorted_list_to_bst(l1));
        assert_eq!(Solution::sorted_list_to_bst(l1), t);
    }
}
