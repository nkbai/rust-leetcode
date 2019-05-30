/*
给定一个链表和一个特定值 x，对链表进行分隔，使得所有小于 x 的节点都在大于或等于 x 的节点之前。

你应当保留两个分区中每个节点的初始相对位置。

示例:

输入: head = 1->4->3->2->5->2, x = 3
输出: 1->2->2->4->3->5
*/
use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        //小于x的
        let mut p1 = Some(Box::new(ListNode::new(0)));
        let mut rp1 = p1.as_mut();
        //大于等x的
        let mut p2 = Some(Box::new(ListNode::new(0)));
        let mut rp2 = p2.as_mut();
        let mut nhead = head;
        while nhead.is_some() {
            let mut n = nhead.unwrap();
            nhead = n.next.take();
            if n.val < x {
                rp1.as_mut().unwrap().next = Some(Box::new(ListNode::new(n.val)));
                rp1 = rp1.unwrap().next.as_mut();
            } else {
                rp2.as_mut().unwrap().next = Some(Box::new(ListNode::new(n.val)));
                rp2 = rp2.unwrap().next.as_mut();
            }
        }
        rp1.unwrap().next = p2.unwrap().next;
        p1.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_list_node;
    #[test]
    fn test_partition() {
        assert_eq!(
            build_list_node(&vec![1, 2, 2, 4, 3, 5]),
            Solution::partition(build_list_node(&vec![1, 4, 3, 2, 5, 2]), 3)
        );
    }
}
