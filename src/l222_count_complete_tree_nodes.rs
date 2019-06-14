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
use core::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
    pub fn treeHeight(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut root = root;
        let mut height = 0;
        //        if let Some(ref r) = root {
        //            height += 1;
        //            root = &r.borrow().borrow().left
        //        }
        return height;
    }
}
