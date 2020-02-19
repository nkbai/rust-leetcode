/*
897. 递增顺序查找树

给定一个树，按中序遍历重新排列树，使树中最左边的结点现在是树的根，并且每个结点没有左子结点，只有一个右子结点。



示例 ：
````
输入：[5,3,6,2,4,null,8,1,null,null,null,7,9]

       5
      / \
    3    6
   / \    \
  2   4    8
/        / \
1        7   9
```
输出：[1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]
```
 1
\
2
\
3
\
4
\
5
\
6
\
7
\
8
\
                 9

```
提示：

给定树中的结点数介于 1 和 100 之间。
每个结点都有一个从 0 到 1000 范围内的唯一整数值。


*/
/*
思路:
递归中序遍历,
1. 先遍历左子树,
 如果左子树为空,则直接追加在当前根上,否则追加在左子树返回的右下角节点上.
2. 自己成为新的根
3. 将自己作为新的根来遍历右子树.
如果右子树为空,新的根仍然是自己,否则是右子树返回节点
*/

struct Solution {}
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut new_root = None;
        Self::inorder_traverse(&mut new_root, root);
        new_root
    }
    pub fn inorder_traverse(
        new_root: &mut Option<Rc<RefCell<TreeNode>>>,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let r = root.unwrap();
        println!("visit:{}", r.borrow().val);
        let left = r.borrow_mut().left.take();
        let mut r2 = Self::inorder_traverse(new_root, left);
        match r2 {
            None => {
                if new_root.is_none() {
                    *new_root = Some(r.clone());
                } else {
                    new_root.as_ref().unwrap().borrow_mut().right = Some(r.clone());
                }
            }
            Some(ref mut new_root) => {
                new_root.borrow_mut().right = Some(r.clone());
            }
        };
        let right = r.borrow_mut().right.take();
        let mut new_root = Some(r);
        let r3 = Self::inorder_traverse(&mut new_root, right);
        if r3.is_none() {
            return new_root;
        } else {
            return r3;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::NULL;
    #[test]
    fn test() {
        let t = crate::share::build_tree_ignore_parent(&vec![
            5, 3, 6, 2, 4, NULL, 8, 1, NULL, NULL, NULL, 7, 9,
        ]);
        println!("t={}", t.as_ref().unwrap().borrow().to_string());
        let r = Solution::increasing_bst(t);
        println!("r={}", r.unwrap().borrow().to_string());
        let r = Solution::increasing_bst(None);
        println!("r2={:?}", r);
    }
}
