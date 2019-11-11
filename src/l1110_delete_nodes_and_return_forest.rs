/*
1110. 删点成林

给出二叉树的根节点 root，树上每个节点都有一个不同的值。

如果节点值在 to_delete 中出现，我们就把该节点从树上删去，最后得到一个森林（一些不相交的树构成的集合）。

返回森林中的每棵树。你可以按任意顺序组织答案。



示例：



输入：root = [1,2,3,4,5,6,7], to_delete = [3,5]
输出：[[1,2,null,4],[6],[7]]


提示：

树中的节点数最大为 1000。
每个节点都有一个介于 1 到 1000 之间的值，且各不相同。
to_delete.length <= 1000
to_delete 包含一些从 1 到 1000、各不相同的值。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/delete-nodes-and-return-forest
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
1.将todelete中的数据转换为map保存
2. 递归遍历整棵树,如果当前节点在待删除列表中,返回None
3. 如果不在待删除列表则返回自身
*/
use crate::share::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
struct Solution;

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut results = Vec::new();
        let mut set = HashSet::new();
        let mut root = root.clone();
        to_delete.iter().for_each(|n| {
            set.insert(*n);
        });
        Self::del_internal(&mut root, &set, true, &mut results);
        results
    }
    fn del_internal(
        node: &mut Option<Rc<RefCell<TreeNode>>>,
        to_delete: &HashSet<i32>,
        is_root: bool,
        results: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) {
        if node.is_none() {
            return;
        }
        let n = node.clone().unwrap();
        let mut next_is_root = false;
        if to_delete.contains(&n.borrow().val) {
            //删除自身,然后构建两颗独立的树
            *node = None; //清空自己
            next_is_root = true;
        } else {
            //我不需要删除,如果我是一棵独立的树就立即保存,否则我已经在一颗树中了,不用管了
            if is_root {
                results.push(Some(n.clone()));
            }
        }
        Self::del_internal(&mut n.borrow_mut().left, to_delete, next_is_root, results);
        Self::del_internal(&mut n.borrow_mut().right, to_delete, next_is_root, results);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let r = build_tree_ignore_parent(&vec![1, 2, 3, 4, 5, 6, 7]);
        let to_delete = vec![3, 5];
        let rs = Solution::del_nodes(r, to_delete);
        rs.iter().for_each(|n| {
            println!("tree: {}", n.clone().unwrap().borrow().to_string());
        });
        //        println!("rs={:?}", rs);
    }
}
