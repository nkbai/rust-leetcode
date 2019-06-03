/*
反转从位置 m 到 n 的链表。请使用一趟扫描完成反转。

说明:
1 ≤ m ≤ n ≤ 链表长度。

示例:

输入: 1->2->3->4->5->NULL, m = 2, n = 4
输出: 1->4->3->2->5->NULL

这个解法反复不停的Box::new,应该可以不用new的,但是对于所有权和借用不是那么熟悉,只能先这么着.
*/
use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut nhead = head;
        let mut rhead = nhead.as_mut();
        let mut cnt = 1;
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut rr = result.as_mut();
        loop {
            if cnt == m {
                //开始反转
                let mut start = Some(Box::new(ListNode::new(rhead.as_mut().unwrap().val)));
                rhead = rhead.unwrap().next.as_mut();
                cnt += 1;
                loop {
                    //题目应该确保链表长度大于n
                    if cnt > n {
                        break;
                    }
                    start = Some(Box::new(ListNode {
                        val: rhead.as_mut().unwrap().val,
                        next: start,
                    }));

                    rhead = rhead.unwrap().next.as_mut();
                    cnt += 1;
                }
                if start.is_some() {
                    rr.as_mut().unwrap().next = start;
                }
                //rr移动到最后
                while rr.as_ref().unwrap().next.is_some() {
                    rr = rr.unwrap().next.as_mut();
                }
                //剩下反复构造的节点的成本
                //                while start.is_some() {
                //                    rr.as_mut().unwrap().next =
                //                        Some(Box::new(ListNode::new(start.as_ref().unwrap().val)));
                //                    rr = rr.unwrap().next.as_mut();
                //                    start = start.unwrap().next.take();
                //                }
            }
            if rhead.is_none() {
                break;
            }
            rr.as_mut().unwrap().next = Some(Box::new(ListNode::new(rhead.as_ref().unwrap().val)));
            rr = rr.unwrap().next.as_mut();
            rhead = rhead.unwrap().next.as_mut();
            cnt += 1;
        }
        result.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_list_node;
    #[test]
    fn test_reverse() {
        assert_eq!(
            build_list_node(&vec![1, 4, 3, 2, 5]),
            Solution::reverse_between(build_list_node(&vec![1, 2, 3, 4, 5]), 2, 4)
        );
        assert_eq!(
            build_list_node(&vec![5, 4, 3, 2, 1]),
            Solution::reverse_between(build_list_node(&vec![1, 2, 3, 4, 5]), 1, 5)
        );
        assert_eq!(
            build_list_node(&vec![1, 2, 3, 4, 5]),
            Solution::reverse_between(build_list_node(&vec![1, 2, 3, 4, 5]), 1, 1)
        );
    }
}
