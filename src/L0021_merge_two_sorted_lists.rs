/*
将两个有序链表合并为一个新的有序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。

示例：

输入：1->2->4, 1->3->4
输出：1->1->2->3->4->4
*/
use crate::share;
use crate::share::buildListNode;
use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let (mut p, mut q) = (l1, l2);
        let mut current = head.as_mut();
        while p.is_some() || q.is_some() {
            let mut v = 0;
            if p.is_some() && q.is_some() {
                if p.as_ref().unwrap().val < q.as_ref().unwrap().val {
                    v = p.as_ref().unwrap().val;
                    p = p.unwrap().next;
                } else {
                    v = q.as_ref().unwrap().val;
                    q = q.unwrap().next;
                }
            } else if let Some(v2) = p {
                v = v2.val;
                p = v2.next;
            } else if let Some(v2) = q {
                v = v2.val;
                q = v2.next;
            }
            let c2 = current.unwrap();
            c2.next = Some(Box::new(ListNode::new(v)));
            current = c2.next.as_mut();
        }
        head.unwrap().next
    }

    pub fn merge_two_lists_2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }
        let mut l1 = l1;
        let mut l2 = l2;

        let mut head = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut head;

        loop {
            if l1.is_none() && l2.is_none() {
                break;
            }
            if l1.is_none() {
                if let Some(node) = cur {
                    node.next = l2
                }
                break;
            }
            if l2.is_none() {
                if let Some(node) = cur {
                    node.next = l1
                }
                break;
            }
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let next = l1.as_mut().unwrap().next.take();
                if let Some(node) = cur {
                    node.next = l1;
                    cur = &mut node.next;
                    l1 = next;
                }
            } else {
                let next = l2.as_mut().unwrap().next.take();
                if let Some(node) = cur {
                    node.next = l2;
                    cur = &mut node.next;
                    l2 = next;
                }
            }
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_two_lists() {
        let l1 = buildListNode(&vec![2, 3, 4]);
        let l2 = buildListNode(&vec![1, 3, 7]);
        let l3 = buildListNode(&vec![1, 2, 3, 3, 4, 7]);
        assert_eq!(l3, Solution::merge_two_lists(l1, l2));
        let l1 = buildListNode(&vec![5]);
        let l2 = buildListNode(&vec![5, 7]);
        let l3 = buildListNode(&vec![5, 5, 7]);
        assert_eq!(l3, Solution::merge_two_lists(l1, l2))
    }
}
