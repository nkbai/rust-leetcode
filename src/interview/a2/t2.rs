/*
给定两个非空链表来代表两个非负整数。数字最高位位于链表开始位置。它们的每个节点只存储单个数字。将这两数相加会返回一个新的链表。



你可以假设除了数字 0 之外，这两个数字都不会以零开头。

进阶:

如果输入链表不能修改该如何处理？换句话说，你不能对列表中的节点进行翻转。

示例:

输入: (7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
输出: 7 -> 8 -> 0 -> 7
*/
use crate::share::ListNode;
use std::cmp::max;

struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut f1 = Vec::new();
        let mut f2 = Vec::new();
        let mut l1 = l1;
        let mut l2 = l2;
        while l1.is_some() {
            let t = l1.clone().unwrap();
            f1.push(t.val as u8);
            l1 = t.next;
        }
        while l2.is_some() {
            let t = l2.clone().unwrap();
            f2.push(t.val as u8);
            l2 = t.next;
        }
        let res = Self::add(&mut f1, &mut f2);
        let mut last = None;
        for i in 0..res.len() {
            last = Some(Box::new(ListNode {
                val: res[i] as i32,
                next: last,
            }))
        }
        last
    }
    fn add(l1: &mut Vec<u8>, l2: &mut Vec<u8>) -> Vec<u8> {
        let max_len: usize = max(l1.len(), l2.len());
        let mut res = Vec::new();
        while l1.len() < max_len {
            l1.insert(0, 0);
        }
        while l2.len() < max_len {
            l2.insert(0, 0);
        }
        let mut c = 0;
        for i in (0..max_len).rev() {
            let t = l1[i] + l2[i] + c;
            res.push(t % 10);
            if t > 9 {
                c = 1;
            } else {
                c = 0;
            }
        }
        if c > 0 {
            res.push(c);
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let l1 = build_list_node(&vec![7, 2, 4, 3]);
        let l2 = build_list_node(&vec![5, 6, 4]);
        let l3 = build_list_node(&vec![7, 8, 0, 7]);
        assert_eq!(l3, Solution::add_two_numbers(l1, l2));
    }
}
