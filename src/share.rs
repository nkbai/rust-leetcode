// Definition for singly-linked list.
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
pub const NULL: i32 = std::i32::MIN;
pub const null: i32 = std::i32::MIN;
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Ord for ListNode {
    fn cmp(&self, other: &ListNode) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
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

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
/*
构造树的时候,按照leetcode的规则来,如果父节点是空,那么就不必 给出子节点.
这样就不是严格意义上的那个2i+1,2i+2,稍微复杂一点
*/
pub fn build_tree_ignore_parent(v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let parent = Vec::new();
    return buld_tree_ignore_parent_internal(v, &parent, 0);
}
fn buld_tree_ignore_parent_internal(
    v: &Vec<i32>,
    parent: &Vec<Rc<RefCell<TreeNode>>>,
    from: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut this_level_len = parent.len();
    if this_level_len == 0 && v.len() == 0 {
        return None; //空树
    } else if this_level_len == 0 {
        let root = Rc::new(RefCell::new(TreeNode {
            val: v[0],
            left: None,
            right: None,
        }));
        let parent = vec![root.clone()];
        buld_tree_ignore_parent_internal(v, &parent, 1);
        return Some(root);
    } else {
        //其他层
        let mut has_left = true;
        let mut this_level = v.iter().skip(from);
        let mut next_level = Vec::new();

        parent.iter().for_each(|r| {
            if !has_left {
                return;
            }
            //处理左右子节点
            let mut cnt = 0;
            loop {
                if cnt >= 2 {
                    break;
                }
                cnt += 1;
                if let Some(c1) = this_level.next() {
                    //如果为空,相当于已经添加到树上了
                    if *c1 != null {
                        let n = Rc::new(RefCell::new(TreeNode {
                            val: *c1,
                            left: None,
                            right: None,
                        }));
                        next_level.push(n.clone());
                        if cnt == 1 {
                            r.borrow_mut().left = Some(n);
                        } else {
                            r.borrow_mut().right = Some(n);
                        }
                    }
                } else {
                    has_left = false;
                }
            }
        });
        if has_left {
            buld_tree_ignore_parent_internal(v, &next_level, from + parent.len() * 2);
        }
        return None;
    }
}

pub fn build_tree(v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_helper(0, v)
}
fn build_tree_helper(i: usize, v: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if i >= v.len() {
        return None;
    }
    if v[i] == NULL {
        return None;
    }
    let left = build_tree_helper(2 * i + 1, v);
    let right = build_tree_helper(2 * i + 2, v);
    Some(Rc::new(RefCell::new(TreeNode {
        val: v[i],
        left,
        right,
    })))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_build_tree() {
        println!("tree={:?}", build_tree(&vec![0, -3, 9, -10, NULL, 5]));
    }
    #[test]
    fn test_build_tree2() {
        println!(
            "tree={:?}",
            build_tree_ignore_parent(&vec![
                4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6,
                5, null, 9, null, null, -1, -4, null, null, null, -2
            ])
        );
    }
}
