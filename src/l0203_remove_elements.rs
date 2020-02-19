/*
删除链表中等于给定值 val 的所有节点。

示例:

输入: 1->2->6->3->4->5->6, val = 6
输出: 1->2->3->4->5
*/

use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();

        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            println!("current={:?}", current);
            if current.as_ref().unwrap().next.as_ref().unwrap().val == val {
                let next = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = next;
            } else {
                current = current.unwrap().next.as_mut();
            }
        }
        if head.as_ref().unwrap().val == val {
            return head.unwrap().next;
        } else {
            return head;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::build_list_node;
    #[test]
    fn it_works() {
        let l1 = build_list_node(&vec![2, 3, 3, 4]);
        let l2 = build_list_node(&vec![2, 4]);
        //        println!(
        //            "l2={:?},\nremove l1={:?}",
        //            l2,
        //            Solution::remove_elements(l1, 3)
        //        );
        assert_eq!(l2, Solution::remove_elements(l1, 3));
        let l1 = build_list_node(&vec![5]);
        assert_eq!(None, Solution::remove_elements(l1, 5))
    }
}
