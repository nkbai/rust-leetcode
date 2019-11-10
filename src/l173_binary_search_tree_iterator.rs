/*

173. 二叉搜索树迭代器
实现一个二叉搜索树迭代器。你将使用二叉搜索树的根节点初始化迭代器。

调用 next() 将返回二叉搜索树中的下一个最小的数。



示例：
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/25/bst-tree.png)


BSTIterator iterator = new BSTIterator(root);
iterator.next();    // 返回 3
iterator.next();    // 返回 7
iterator.hasNext(); // 返回 true
iterator.next();    // 返回 9
iterator.hasNext(); // 返回 true
iterator.next();    // 返回 15
iterator.hasNext(); // 返回 true
iterator.next();    // 返回 20
iterator.hasNext(); // 返回 false


提示：

next() 和 hasNext() 操作的时间复杂度是 O(1)，并使用 O(h) 内存，其中 h 是树的高度。
你可以假设 next() 调用总是有效的，也就是说，当调用 next() 时，BST 中至少存在一个下一个最小的数。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-search-tree-iterator
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
O(1)复杂度的思路:
1. 使用map来保存数据 key到映射value以及双向链表中的元素
2. get 查询,如果查询得到,从双向链表中移除并添加到尾部
3. put 如果满了,则移除双向链表首部元素,同时插入map,添加到尾部
手工模拟双向链表.
双向链表操作直接借鉴自标准库
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    path: Vec<Rc<RefCell<TreeNode>>>, //保存当前路径
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut bst = BSTIterator { path: Vec::new() };
        bst.push_left(root);
        bst
    }
    fn push_left(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap();
        self.path.push(r.clone());
        self.push_left(r.borrow().left.clone());
    }
    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        let l = self.path.pop().unwrap();
        let val = l.borrow().val;
        //左节点已经处理过了,右节点需要处理
        let r = l.borrow().right.clone();
        self.push_left(r);
        val
    }

    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        self.path.len() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;

    #[test]
    fn test_lru() {
        let t = build_tree_ignore_parent(&vec![7, 3, 15, null, null, 9, 20]);
        let mut bst = BSTIterator::new(t);
        assert_eq!(bst.next(), 3);
        assert_eq!(bst.has_next(), true);
        assert_eq!(bst.next(), 7);
        assert_eq!(bst.has_next(), true);
        assert_eq!(bst.next(), 9);
        assert_eq!(bst.has_next(), true);
        assert_eq!(bst.next(), 15);
        assert_eq!(bst.has_next(), true);
        assert_eq!(bst.next(), 20);
        assert_eq!(bst.has_next(), false);
    }
}
