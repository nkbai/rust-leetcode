#[allow(dead_code)]
use crate::share::ListNode;
struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let result = None;
        //        Solution::reverseListRecursive(head, result)
        Solution::reverse_list_for(head, result)
    }
    //    fn reverse_internal(head: &mut Option<Box<ListNode>>) -> retHelp {
    //        if head.is_none() {
    //            return retHelp {
    //                cur: &mut None,
    //                head: &mut None,
    //            };
    //        }
    //        println!("enter :{:?}", head.as_ref().unwrap());
    //        //传进来的head不可修改,只能创建新的
    //        let mut cur = head;
    //        let mut next = cur.as_mut().unwrap().next.take();
    //        let mut result = Solution::reverse_internal(&mut next);
    //        if result.cur.is_none() && result.head.is_none() {
    //            //他的next是空,cur就是新head了
    //            return retHelp {
    //                cur: &mut None,
    //                head: cur,
    //            };
    //        }else if result.cur.is_none() && result.head.is_some(){
    //            return retHelp{
    //                cur:
    //            }
    //        }
    //        result.cur.as_mut().unwrap().next = *cur;
    //        println!("return :{:?}", newhead);
    //        &mut newhead.as_ref().unwrap().next
    //    }
    #[allow(dead_code)]
    fn reverse_list_recursive(
        list: Option<Box<ListNode>>,
        result: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match list {
            Some(val) => {
                let result = Some(Box::new(ListNode {
                    val: val.val, //通过复制Node的方式而不是我做的那种原地替换的方式
                    next: result,
                }));
                Solution::reverse_list_recursive(val.next, result)
            }
            None => result,
        }
    }
    fn reverse_list_for(
        head: Option<Box<ListNode>>,
        result: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result2 = result;
        let mut head2 = head;
        loop {
            match head2 {
                Some(val) => {
                    result2 = Some(Box::new(ListNode {
                        val: val.val,
                        next: result2,
                    }));
                    head2 = val.next;
                }
                None => break,
            }
        }

        result2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share;
    #[test]
    fn test_reverse_list() {
        let l1 = share::build_list_node(&vec![1, 2, 3, 4, 5]);
        let l2 = share::build_list_node(&vec![5, 4, 3, 2, 1]);
        assert_eq!(l1, Solution::reverse_list(l2));
        assert_eq!(
            share::build_list_node(&vec![1]),
            Solution::reverse_list(share::build_list_node(&vec![1]))
        )
    }
}
