/*
143. 重排链表
给定一个单链表 L：L0→L1→…→Ln-1→Ln ，
将其重新排列后变为： L0→Ln→L1→Ln-1→L2→Ln-2→…

你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。

示例 1:

给定链表 1->2->3->4, 重新排列为 1->4->2->3.
示例 2:

给定链表 1->2->3->4->5, 重新排列为 1->5->2->4->3.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/reorder-list
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
因为是单向链表,所以先以收集后半部分的节点,
然后进行插入操作
*/
use crate::share::ListNode;

struct Solution {}
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut cnt = 0;
        let mut v = Vec::new();
        let mut n = head.as_ref();
        while n.is_some() {
            cnt += 1;
            n = n.unwrap().next.as_ref();
        }
        let right = cnt / 2;
        let mut i = 0;
        //        let head=head.unwrap();
        let mut n = head.as_mut();
        while n.is_some() {
            i += 1;
            let mut n2 = n.unwrap();
            //            println!("n2={},i={},cnt={},right={}", n2.val, i, cnt, right);
            if i >= cnt - right {
                let n3 = n2.next.take();
                if n3.is_some() {
                    v.push(n3);
                    n = v.last_mut().unwrap().as_mut();
                } else {
                    n = None
                }
            } else {
                n = n2.next.as_mut();
            }
        }
        //        println!("head={:?}\n,v={:?}", head, v);
        let mut n = head.as_mut();
        let mut i = 0;
        while v.len() > 0 && n.is_some() {
            let mut n2 = n.unwrap();
            let mut n3 = n2.next.take();
            let mut vi = v.pop().unwrap();
            vi.as_mut().unwrap().next = n3;
            n2.next = vi;
            n = n2.next.as_mut().unwrap().next.as_mut();
        }
        //        println!("head={:?}", head);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let mut t0 = build_list_node(&vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut t0);
        assert_eq!(t0, build_list_node(&vec![1, 5, 2, 4, 3]));
        let mut t0 = build_list_node(&vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut t0);
        assert_eq!(t0, build_list_node(&vec![1, 4, 2, 3]));
        let mut t0 = build_list_node(&vec![1]);
        Solution::reorder_list(&mut t0);
        assert_eq!(t0, build_list_node(&vec![1]));
        let mut t0 = build_list_node(&vec![]);
        Solution::reorder_list(&mut t0);
        assert_eq!(t0, build_list_node(&vec![]));
    }
}
