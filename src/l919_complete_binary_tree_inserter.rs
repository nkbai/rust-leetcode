/*
919. 完全二叉树插入器

完全二叉树是每一层（除最后一层外）都是完全填充（即，结点数达到最大）的，并且所有的结点都尽可能地集中在左侧。

设计一个用完全二叉树初始化的数据结构 CBTInserter，它支持以下几种操作：

CBTInserter(TreeNode root) 使用头结点为 root 的给定树初始化该数据结构；
CBTInserter.insert(int v) 将 TreeNode 插入到存在值为 node.val = v  的树中以使其保持完全二叉树的状态，并返回插入的 TreeNode 的父结点的值；
CBTInserter.get_root() 将返回树的头结点。


示例 1：

输入：inputs = ["CBTInserter","insert","get_root"], inputs = [[[1]],[2],[]]
输出：[null,1,[1,2]]
示例 2：

输入：inputs = ["CBTInserter","insert","insert","get_root"], inputs = [[[1,2,3,4,5,6]],[7],[8],[]]
输出：[null,3,4,[1,2,3,4,5,6,7,8]]


提示：

最初给定的树是完全二叉树，且包含 1 到 1000 个结点。
每个测试用例最多调用 CBTInserter.insert  操作 10000 次。
给定结点或插入结点的每个值都在 0 到 5000 之间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/complete-binary-tree-inserter
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
struct中保存root以及当前可插入的子节点,这是一个队列.
根据题意,最初的队列一定不为空,那么里面包含着所有可以插入子节点的节点
这样insert的时候只需要从队列中弹出第一个,将节点作为他的孩子节点,同时将新节点也插入队尾

复杂度分析:
时间复杂度:
insert O(1)
空间复杂度 O(N) 队列中需要保存N/2个节点
*/
use crate::share::TreeNode;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    q: Vec<Rc<RefCell<TreeNode>>>, //左右子树至少有一个为空的那些节点
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut all_nodes = Vec::new();
        let mut q = Vec::new();
        all_nodes.push(root.clone().unwrap()); //root肯定不会为空
        while all_nodes.len() > 0 {
            let r = all_nodes.remove(0);
            let left = r.borrow().left.clone();
            let right = r.borrow().right.clone();
            if left.is_none() || right.is_none() {
                q.push(r.clone());
            }
            if left.is_some() {
                all_nodes.push(left.unwrap());
            }
            if right.is_some() {
                all_nodes.push(right.unwrap());
            }
        }
        CBTInserter { root, q }
    }

    fn insert(&mut self, v: i32) -> i32 {
        let new_node = Rc::new(RefCell::new(TreeNode {
            val: v,
            left: None,
            right: None,
        }));
        let f = self.q[0].clone();
        let val = f.borrow().val;
        if f.borrow().left.is_none() {
            f.borrow_mut().left = Some(new_node.clone());
        } else if f.borrow_mut().right.is_none() {
            f.borrow_mut().right = Some(new_node.clone());
            //右边有子节点,说明满了,移除
            self.q.remove(0);
        }
        self.q.push(new_node);
        val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        let t = build_tree_ignore_parent(&vec![1, 2, 3, 4, 5, 6]);
        let mut c = CBTInserter::new(t);
        assert_eq!(c.insert(7), 3);
        assert_eq!(c.insert(8), 4);
        println!("r={:?}", c.get_root());
    }
}
