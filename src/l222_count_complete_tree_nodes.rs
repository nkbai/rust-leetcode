/*
https://leetcode-cn.com/problems/count-complete-tree-nodes/
222. 完全二叉树的节点个数


给出一个完全二叉树，求出该树的节点个数。

说明：

完全二叉树的定义如下：在完全二叉树中，除了最底层节点可能没填满外，其余每层节点数都达到最大值，并且最下面一层的节点都集中在该层最左边的若干位置。若最底层为第 h 层，则该层包含 1~ 2h 个节点。

示例:

输入:
    1
   / \
  2   3
 / \  /
4  5 6

输出: 6

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/count-complete-tree-nodes
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


*/

/*
思路:
1. 先左边走最左边路线,求出树高
2. 再走右子树最左边路线,如果两者树高相等,那就说明满二叉树缺口刚刚在右子树,否则在左子树.
3. 统计该层我的编号(从0起),如果上一层我的编号是3,那么下一层如果走左边我的编号就是2*3,走右边我的编号就是2*3+1
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut root = root;
        let mut lastnum = 0;
        let mut height = 0;
        if root.is_none() {
            return 0;
        }
        while let Some(r) = root {
            let hl = Solution::treeHeight(r.as_ref().clone().borrow().left.clone());
            let hr = Solution::treeHeight(r.as_ref().clone().borrow().right.clone());
            if hl == 0 {
                break; //走到底了
            }
            if hl == hr {
                //说明满二叉树缺的部分肯定在右边
                root = r.as_ref().clone().borrow().right.clone();
                lastnum = 2 * lastnum + 1;
            } else {
                root = r.as_ref().clone().borrow().left.clone();
                lastnum = 2 * lastnum;
            }
            height += 1;
        }
        2.0_f32.powi(height) as i32 + lastnum
    }
    pub fn treeHeight(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut root = root;
        let mut height = 0;
        while let Some(r) = root {
            height += 1;
            root = r.as_ref().clone().borrow().left.clone();
        }
        return height;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    #[test]
    fn test_count_nodes() {
        let t = build_tree(&vec![1]);
        assert_eq!(1, Solution::count_nodes(t));
        let t = build_tree(&vec![]);
        assert_eq!(0, Solution::count_nodes(t));
        let t = build_tree(&vec![1, 2]);
        assert_eq!(2, Solution::count_nodes(t));
        let t = build_tree(&vec![1, 2, 3]);
        assert_eq!(3, Solution::count_nodes(t));
        let t = build_tree(&vec![1, 2, 3, 4]);
        println!("t={:?}", t);
        assert_eq!(4, Solution::count_nodes(t));
        let t = build_tree(&vec![1, 2, 3, 4, 5]);
        assert_eq!(5, Solution::count_nodes(t));
        let t = build_tree(&vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(6, Solution::count_nodes(t));
    }
}
