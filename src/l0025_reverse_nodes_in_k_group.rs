/*
https://leetcode-cn.com/problems/reverse-nodes-in-k-group/

给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。

k 是一个正整数，它的值小于或等于链表的长度。

如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。

示例 :

给定这个链表：1->2->3->4->5

当 k = 2 时，应当返回: 2->1->4->3->5

当 k = 3 时，应当返回: 3->2->1->4->5

说明 :

你的算法只能使用常数的额外空间。
你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。

参考#92题目,循环分组即可
说明:
因为对于所有权转移借用这一套不是很熟悉,这里不是常数额外空间占用的.
*/
use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut nhead = head;
        let mut rhead = nhead.as_mut();
        let mut cnt = 0;
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut rr = result.as_mut(); //保存的分组翻转的链接
        loop {
            //只要还有就继续分组
            if cnt % k == 0 && rhead.is_some() {
                //新一轮的开始
                //开始反转,要考虑剩余节点不够k个的时候,保持顺序,
                //这里使用一个vector来保存
                let mut start = Some(Box::new(ListNode::new(rhead.as_mut().unwrap().val)));
                rhead = rhead.unwrap().next.as_mut();
                cnt += 1;
                let mut cntk: bool;
                loop {
                    //题目应该确保链表长度大于n
                    if cnt % k == 0 {
                        //下一个分组
                        cntk = true;
                        break;
                    }
                    //剩下的不够k个了
                    if rhead.is_none() {
                        cntk = false;
                        break;
                    }
                    start = Some(Box::new(ListNode {
                        val: rhead.as_mut().unwrap().val,
                        next: start,
                    }));

                    rhead = rhead.unwrap().next.as_mut();
                    cnt += 1;
                }
                /**/
                if cntk {
                    //够k个,不用再次翻转了

                    if start.is_some() {
                        rr.as_mut().unwrap().next = start;
                    }
                    //rr移动到最后
                    while rr.as_ref().unwrap().next.is_some() {
                        rr = rr.unwrap().next.as_mut();
                    }
                } else {
                    //不够k个,需要按照start的逆序存进rr
                    let mut vec = Vec::new();
                    while start.is_some() {
                        vec.push(start.as_ref().unwrap().val);
                        start = start.as_mut().unwrap().next.take();
                    }
                    while let Some(v) = vec.pop() {
                        let node = Some(Box::new(ListNode::new(v)));
                        rr.as_mut().unwrap().next = node;
                        rr = rr.unwrap().next.as_mut();
                    }
                }
            } else {
                break;
            }
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
            build_list_node(&vec![2, 1, 4, 3, 5]),
            Solution::reverse_k_group(build_list_node(&vec![1, 2, 3, 4, 5]), 2)
        );
        assert_eq!(
            build_list_node(&vec![3, 2, 1, 4, 5]),
            Solution::reverse_k_group(build_list_node(&vec![1, 2, 3, 4, 5]), 3)
        );
        assert_eq!(
            build_list_node(&vec![1, 2, 3, 4, 5]),
            Solution::reverse_k_group(build_list_node(&vec![1, 2, 3, 4, 5]), 1)
        );
    }
}
