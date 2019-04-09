/*
给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。

示例：

给定一个链表: 1->2->3->4->5, 和 n = 2.

当删除了倒数第二个节点后，链表变为 1->2->3->5.
说明：

给定的 n 保证是有效的。

进阶：

你能尝试使用一趟扫描实现吗？
*/
use crate::share::build_list_node;
use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut h = head;
        let mut current = h.as_mut();
        while current.is_some() {
            len += 1;
            current = current.unwrap().next.as_mut();
        }
        let pos = len - n - 1; //前一个
        if pos < 0 {
            return h.unwrap().next;
        }
        let mut i = 0;
        let mut current = h.as_mut();
        while current.is_some() {
            if i == pos {
                //真的难写..
                let next = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = next;

                //                current.as_mut().unwrap().next =
                //                    current.as_mut().unwrap().next.as_mut().unwrap().next;
                break;
            } else {
                i += 1;
                current = current.unwrap().next.as_mut();
            }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_nth_from_end() {
        let l1 = build_list_node(&vec![1, 2, 3, 4, 5]);
        let l2 = build_list_node(&vec![1, 2, 3, 5]);
        assert_eq!(l2, Solution::remove_nth_from_end(l1, 2));
        let l1 = build_list_node(&vec![1, 2, 3, 4, 5]);
        let l2 = build_list_node(&vec![1, 2, 3, 4]);
        assert_eq!(l2, Solution::remove_nth_from_end(l1, 1));
    }
}
