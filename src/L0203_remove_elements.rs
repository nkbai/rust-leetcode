/*
删除链表中等于给定值 val 的所有节点。

示例:

输入: 1->2->6->3->4->5->6, val = 6
输出: 1->2->3->4->5
*/
use crate::share;
use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();
        let i = 0;

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
    #[test]
    fn it_works() {
        let l1 = share::buildListNode(&vec![2, 3, 3, 4]);
        let l2 = share::buildListNode(&vec![2, 4]);
        assert_eq!(l2, Solution::remove_elements(l1, 3));
        let l1 = share::buildListNode(&vec![5]);
        assert_eq!(None, Solution::remove_elements(l1, 5))
    }
}
