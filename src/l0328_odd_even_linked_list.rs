/*
给定一个单链表，把所有的奇数节点和偶数节点分别排在一起。请注意，这里的奇数节点和偶数节点指的是节点编号的奇偶性，而不是节点的值的奇偶性。

请尝试使用原地算法完成。你的算法的空间复杂度应为 O(1)，时间复杂度应为 O(nodes)，nodes 为节点总数。

示例 1:

输入: 1->2->3->4->5->NULL
输出: 1->3->5->2->4->NULL
示例 2:

输入: 2->1->3->5->6->4->7->NULL
输出: 2->3->6->7->1->5->4->NULL
说明:

应当保持奇数节点和偶数节点的相对顺序。
链表的第一个节点视为奇数节点，第二个节点视为偶数节点，以此类推。
*/

/*
我的思路:
走一遍即可
一个寄指针,一个偶指针
如果当前位置是奇数就移动到奇数指针下,否则就移动到偶数指针下
因为是指针的移动,所以空间复杂度是O(1)
*/
use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nhead = head;
        let mut odd = Some(Box::new(ListNode::new(0)));
        let mut even = Some(Box::new(ListNode::new(0)));
        let mut rodd = odd.as_mut();
        let mut reven = even.as_mut();
        let mut pos = 1;
        while nhead.is_some() {
            //先取走下一个
            let next = nhead.as_mut().unwrap().next.take();
            if pos % 2 == 1 {
                rodd.as_mut().unwrap().next = nhead;
                rodd = rodd.unwrap().next.as_mut();
            } else {
                reven.as_mut().unwrap().next = nhead;
                reven = reven.unwrap().next.as_mut();
            }
            nhead = next;
            pos += 1
        }

        rodd.as_mut().unwrap().next = even.unwrap().next;
        odd.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_list_node;
    #[test]
    fn test_odd_even_list() {
        let l1 = build_list_node(&vec![1, 2, 3, 4, 5]);
        let l2 = build_list_node(&vec![1, 3, 5, 2, 4]);
        assert_eq!(Solution::odd_even_list(l1), l2);
        let l1 = build_list_node(&vec![2, 1, 3, 5, 6, 4, 7]);
        let l2 = build_list_node(&vec![2, 3, 6, 7, 1, 5, 4]);
        assert_eq!(Solution::odd_even_list(l1), l2);
        let l1 = build_list_node(&vec![]);
        let l2 = build_list_node(&vec![]);
        assert_eq!(Solution::odd_even_list(l1), l2);
        let l1 = build_list_node(&vec![2]);
        let l2 = build_list_node(&vec![2]);
        assert_eq!(Solution::odd_even_list(l1), l2);
    }
}
