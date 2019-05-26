use crate::share::build_list_node;
use crate::share::ListNode;
use std::collections::BinaryHeap;

struct Solution {}
/*
思路:
1. 针对k个链表,按照头部最小堆排序
2. 从最小的链表中取一个,如果第二个小于等于第二小的链,继续取
3. 将取出的链重新插入堆中参与排序
*/
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        //        println!("lists={:#?}",lists);
        let mut h = BinaryHeap::new();
        for x in lists.iter() {
            h.push(x.clone())
        }
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();
        while let Some(n) = h.pop() {
            //            println!("n={:#?}",n);
            if n.is_none() {
                continue;
            }
            let l = n.unwrap();
            let next = ListNode::new(l.val);
            let v = current.unwrap();
            v.next = Some(Box::new(next));
            current = v.next.as_mut();
            h.push(l.next)
        }
        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_k_sorted_lists() {
        /*
            输入:
        [
          1->4->5,
          1->3->4,
          2->6
        ]
        输出: 1->1->2->3->4->4->5->6
        */
        //        let l1 = build_list_node(&vec![1, 32, 3]);
        let l2 = Solution::merge_k_lists(vec![
            build_list_node(&vec![1, 4, 5]),
            build_list_node(&vec![1, 3, 4]),
            build_list_node(&vec![2, 6]),
        ]);
        assert_eq!(l2, build_list_node(&vec!(1, 1, 2, 3, 4, 4, 5, 6)));
        //        assert_eq!(3,5);
    }
}
