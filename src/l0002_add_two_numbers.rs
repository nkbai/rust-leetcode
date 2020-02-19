/*
给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。

如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例：

输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
原因：342 + 465 = 807
*/

use crate::share::ListNode;

struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut p, mut q, mut carry) = (l1, l2, 0);
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();
        while p.is_some() || q.is_some() {
            let mut sum = carry;
            if let Some(v) = p {
                sum += v.val;
                p = v.next;
            }
            if let Some(v) = q {
                sum += v.val;
                q = v.next;
            }
            carry = sum / 10;
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(sum % 10)));
            current = v.next.as_mut();
        }
        if carry > 0 {
            let v = current.unwrap();
            v.next = Some(Box::new(ListNode::new(carry)));
        }
        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::build_list_node;
    #[test]
    fn test_sum() {
        let l1 = build_list_node(&vec![2, 4, 3]);
        let l2 = build_list_node(&vec![5, 6, 4]);
        let l3 = build_list_node(&vec![7, 0, 8]);
        assert_eq!(l3, Solution::add_two_numbers(l1, l2));
        let l1 = build_list_node(&vec![5]);
        let l2 = build_list_node(&vec![5]);
        let l3 = build_list_node(&vec![0, 1]);
        assert_eq!(l3, Solution::add_two_numbers(l1, l2))
    }
}
