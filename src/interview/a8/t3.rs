/*

*/
/*
思路:
 一次走两步,然后交换.

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
            //如果第一个就是空,直接就结束了
            let mut n1 = chead.as_mut().unwrap().next.take();
            if n1.is_none() {
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

    #[test]
    fn test() {
        let l = crate::share::build_list_node(&vec![1, 2, 3, 4]);
        let l2 = Solution::swap_pairs(l);
        assert_eq!(l2, crate::share::build_list_node(&vec![2, 1, 4, 3]));
    }
}
