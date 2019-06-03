/*
给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。

示例 1:

输入: 1->1->2
输出: 1->2
示例 2:

输入: 1->1->2->3->3
输出: 1->2->3
*/

use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();

        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            if current.as_ref().unwrap().val == current.as_ref().unwrap().next.as_ref().unwrap().val
            {
                let next = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = next;
            } else {
                current = current.unwrap().next.as_mut();
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::build_list_node;
    #[test]
    fn it_works() {
        let l1 = build_list_node(&vec![2, 3, 3, 4]);
        let l2 = build_list_node(&vec![2, 3, 4]);
        assert_eq!(l2, Solution::delete_duplicates(l1));
        let l1 = build_list_node(&vec![5]);
        let l2 = build_list_node(&vec![5]);
        assert_eq!(l2, Solution::delete_duplicates(l1))
    }
}
