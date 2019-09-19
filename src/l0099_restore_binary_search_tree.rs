/*
99. 恢复二叉搜索树
二叉搜索树中的两个节点被错误地交换。

请在不改变其结构的情况下，恢复这棵树。

示例 1:

输入: [1,3,null,null,2]

1
/
3
\
2

输出: [3,1,null,null,2]

3
/
1
\
2
示例 2:

输入: [3,1,4,null,null,2]

  3
 / \
1   4
/
2

输出: [2,1,4,null,null,3]

  2
 / \
1   4
/
 3
进阶:

使用 O(n) 空间复杂度的解法很容易实现。
你能想出一个只使用常数空间的解决方案吗？


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/recover-binary-search-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

/*
思路:
采用中序遍历,题目中明确只交换了两个节点,多次交换的情况不考虑.
那么无序部分一定是某棵子树中左子树中某一个节点(包含root)应该放在右子树中.
1. 用preNode表示遍历到当前节点时的前一个节点,正常情况下,他应该小于当前节点
2. 如果遍历到preNode大于等于自身的情况,说明preNode错了,这是第一个错误节点
3. 在找到第一个错误节点的情况下,如果还出现preNode大于等于自身的情况,说明这是第二个错误节点
满足条件三的可能会出现两次以上,
只出现一次的情况:
失序情况, 父节点和右子节点直接调换
两次的情况:
其他调换
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
struct Arg {
    first_node: Option<Rc<RefCell<TreeNode>>>,
    second_node: Option<Rc<RefCell<TreeNode>>>,
    pre_node: Option<Rc<RefCell<TreeNode>>>,
    pre_node_value: i32,
}

impl Solution {
    //没有self,只好用这种方式来传参数了,比较丑陋
    fn recover_internal(a: &mut Arg, root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut r = root.as_ref().unwrap().borrow_mut();
        let v = r.val;
        Solution::recover_internal(a, &mut r.left);
        if a.first_node.is_none() && a.pre_node.is_some() && a.pre_node_value >= v {
            a.first_node = a.pre_node.clone();
        }
        if a.first_node.is_some() && a.pre_node_value >= v {
            a.second_node = root.clone();
        }
        a.pre_node = root.clone();
        a.pre_node_value = r.val;
        Solution::recover_internal(a, &mut r.right)
    }
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut a = Arg {
            first_node: None,
            second_node: None,
            pre_node: None,
            pre_node_value: 0,
        };
        Solution::recover_internal(&mut a, root);
        let t = a.first_node.as_mut().unwrap().borrow_mut().val;
        a.first_node.as_mut().unwrap().borrow_mut().val =
            a.second_node.as_mut().unwrap().borrow_mut().val;
        a.second_node.as_mut().unwrap().borrow_mut().val = t;
        return;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_num_trees() {
        let mut r = build_tree(&vec![1, 3, NULL, NULL, 2]);
        Solution::recover_tree(&mut r);
        println!("r={:?}", r);
        let mut r = build_tree(&vec![3, 1, 4, NULL, NULL, 2]);
        Solution::recover_tree(&mut r);
        println!("r={:?}", r);
    }
}
