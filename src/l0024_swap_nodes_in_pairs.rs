/*
给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。

你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。



示例:

给定 1->2->3->4, 你应该返回 2->1->4->3.
*/
use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        /*
        用一个新的head，指向开始，每次前移两个
        */
        let mut newhead = Some(Box::new(ListNode { val: 0, next: head }));
        let mut chead = &mut newhead;
        loop {
            let mut n1 = chead.as_mut().unwrap().next.take();
            if n1.is_none() {
                //如果第一个就是空,直接就结束了
                break;
            }
            let mut n2 = n1.as_mut().unwrap().next.take();
            //如果链表只剩下一个元素了,结束即可,不用调换
            if n2.is_none() {
                //                要还回去
                chead.as_mut().unwrap().next = n1;
                break;
            }
            n1.as_mut().unwrap().next = n2.as_mut().unwrap().next.take();
            n2.as_mut().unwrap().next = n1;
            chead.as_mut().unwrap().next = n2;
            chead = &mut chead.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        newhead.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_swap_pairs() {
        assert_eq!(
            Solution::swap_pairs(build_list_node(&vec![1, 2, 3, 4])),
            build_list_node(&vec![2, 1, 4, 3]),
        );
        assert_eq!(
            Solution::swap_pairs(build_list_node(&vec![1])),
            build_list_node(&vec![1]),
        )
    }
}
