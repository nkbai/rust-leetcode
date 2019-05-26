// Definition for singly-linked list.
use std::cmp::Ordering;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Ord for ListNode{
    fn cmp(&self,other:&ListNode) -> Ordering{
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode{
    fn partial_cmp(&self,other:&ListNode) ->Option<Ordering > {
        Some(self.cmp(other))
    }
}
pub fn build_list_node(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut();
    for i in v {
        let next = ListNode::new(*i);
        let v = current.unwrap();
        //        let Some(v) = current;
        v.next = Some(Box::new(next));
        current = v.next.as_mut();
    }
//    println!("head={:#?}",head);
    head.unwrap().next
}
