/*
652. 寻找重复的子树
给定一棵二叉树，返回所有重复的子树。对于同一类的重复子树，你只需要返回其中任意一棵的根结点即可。

两棵树重复是指它们具有相同的结构以及相同的结点值。

示例 1：

        1
       / \
      2   3
     /   / \
    4   2   4
       /
      4
下面是两个重复的子树：

      2
     /
    4
和

    4
因此，你需要以列表的形式返回上述重复子树的根结点。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-duplicate-subtrees
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
将每颗子树都序列化成字符串,然后作为map的key保存起来,
如果遇到重复,则就是要找的
*/
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::share::TreeNode;

struct Solution {}

impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut v = Vec::new();
        let mut m = HashMap::new();
        Self::internal(root, &mut m, &mut |n| {
            v.push(Some(n));
        });
        v
    }
    //返回当前子树的字符串表示
    fn internal(
        root: Option<Rc<RefCell<TreeNode>>>,
        m: &mut HashMap<String, i32>,
        f: &mut impl FnMut(Rc<RefCell<TreeNode>>),
    ) -> String {
        if root.is_none() {
            return String::new();
        }
        let r = root.unwrap();
        let left = Self::internal(r.borrow().left.clone(), m, f);
        let right = Self::internal(r.borrow().right.clone(), m, f);
        let cur = format!("{},{},{}", r.borrow().val, left, right);
        let cnt = m.entry(cur.clone()).or_default();
        if *cnt == 1 {
            f(r);
        }
        *cnt = *cnt + 1; //多次重复不记录,初次出现也不关心
        return cur;
    }
}

#[cfg(test)]
mod test {
    use crate::share::*;

    use super::*;

    #[test]
    fn test_find_duplcates_tree() {
        let t = Solution::find_duplicate_subtrees(build_tree_ignore_parent(&vec![3, 4, 5, 2, 2]));
        // println!("t={:?}", t);
        let t = Solution::find_duplicate_subtrees(build_tree_ignore_parent(&vec![
            1, 2, 3, 4, null, 2, 4, null, null, 4,
        ]));
        println!("t={:?}", t);
    }
}
