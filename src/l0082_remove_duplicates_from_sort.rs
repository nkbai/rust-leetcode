/*
给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。

示例 1:

输入: 1->2->3->3->4->4->5
输出: 1->2->5
示例 2:

输入: 1->1->1->2->3
输出: 2->3
*/

/*
思路:
如果是无序的链表那就有点麻烦,需要用到map进行过滤.
但是因为是有序链表,那么只要记住当前值,
如果只有下一个新来的值和当前的不一样,那么把替换当前值
当前值是否加入链表,只要他没有重复出现即可.
*/

use crate::share::ListNode;
struct Solution {}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        struct Current {
            val: i32, //当前值是多少
            cnt: i32, //出现了多少次
        }
        let mut nhead = Some(Box::new(ListNode::new(0)));
        let mut rnhead = nhead.as_mut();
        let mut head2 = head;
        let mut rhead = head2.as_mut();
        let mut cur = Current { val: 0, cnt: 0 };
        while rhead.is_some() {
            if cur.cnt == 0 {
                //初始化,直接给一个有效值
                cur = Current {
                    val: rhead.as_ref().unwrap().val,
                    cnt: 1,
                }
            } else if cur.val != rhead.as_mut().unwrap().val {
                //找到了不同值,根据计数决定要不要加进链表
                if cur.cnt == 1 {
                    rnhead.as_mut().unwrap().next = Some(Box::new(ListNode::new(cur.val)));
                    rnhead = rnhead.unwrap().next.as_mut();
                }
                cur = Current {
                    val: rhead.as_ref().unwrap().val,
                    cnt: 1,
                }
            } else {
                //和上一个相同,继续
                cur.cnt += 1;
            }
            rhead = rhead.unwrap().next.as_mut()
        }
        //最后一个数只有一个,也要添加进来
        if cur.cnt == 1 {
            rnhead.as_mut().unwrap().next = Some(Box::new(ListNode::new(cur.val)));
            //rnhead = rnhead.unwrap().next.as_mut(); todo 这个赋值真的没有用? 编译器分析出来的
        }
        nhead.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_list_node;

    #[test]
    fn test_delete_duplicates() {
        assert_eq!(
            Solution::delete_duplicates(build_list_node(&vec![1, 2, 3, 3, 4, 4, 5])),
            build_list_node(&vec![1, 2, 5])
        );
    }
}
