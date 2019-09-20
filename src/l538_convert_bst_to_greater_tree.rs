/*
538. 把二叉搜索树转换为累加树
给定一个二叉搜索树（Binary Search Tree），把它转换成为累加树（Greater Tree)，使得每个节点的值是原来的节点值加上所有大于它的节点值之和。

例如：
```
输入: 二叉搜索树:
              5
            /   \
           2     13

输出: 转换为累加树:
             18
            /   \
          20     13
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/convert-bst-to-greater-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
题目类型:简单
所以按照简单的来
1. 遍历收集所有的节点值
2. 排序
3. 倒着收集,建立map
4. 再次遍历树,将每个节点的值更新
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        //1. 收集所有数据
        let mut v = Vec::new();
        Self::traversal(root.clone(), &mut |r| {
            v.push(r.borrow().val);
        });
        //2. 排序,从大到小
        v.sort();
        println!("v={:?}", v);
        let mut m = BTreeMap::new();
        m.insert(std::i32::MAX, 0);
        /*

        //第一个是上一个对应的值,第二个则是要放入map的累加值
        考虑到存在重复值的情况,那么累加的上海不能加
        比如[1,2,2,3]
        那么map中放入的应该是[[3,0],[2,3],[1,7]]

        */
        let mut cur = (std::i32::MAX, 0);
        v.iter().rev().for_each(|v| {
            println!("v={},cur={:?}", v, cur);
            if *v < cur.0 {
                println!("insert {}=>{}", *v, cur.1);
                m.insert(*v, cur.1);
                cur = (*v, cur.1 + v);
            } else {
                //出现了重复的值
                cur.1 += *v;
            }
        });
        //最后一个也是重复的,比如[1,1,2,3]
        if v.len() > 0 && !m.contains_key(&v[0]) {
            m.insert(v[0], cur.1);
        }
        //更新树的值
        Self::traversal(root.clone(), &mut |r| {
            //必定存在一个有效值
            let v = r.borrow().val;
            let c = m.get(&v);
            r.borrow_mut().val = *c.expect("must exist in map") + v;
        });
        root
    }
    fn traversal(root: Option<Rc<RefCell<TreeNode>>>, f: &mut impl FnMut(Rc<RefCell<TreeNode>>)) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap();
        f(r.clone());
        Self::traversal(r.borrow().left.clone(), f);
        Self::traversal(r.borrow_mut().right.clone(), f);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order_bottom() {
        //        assert_eq!(
        //            Solution::convert_bst(build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7])),
        //            build_tree(&vec![])
        //        );
        //        assert_eq!(
        //            Solution::convert_bst(build_tree(&vec![2, 1, 3])),
        //            build_tree(&vec![5, 6, 3])
        //        );
        assert_eq!(
            Solution::convert_bst(build_tree(&vec![2, 1, 3, 2, 1, 3])),
            build_tree(&vec![8, 11, 3, 8, 11, 3])
        );
        //        assert_eq!(
        //            Solution::convert_bst(build_tree(&vec![
        //                1, 2, 3, 4, NULL, 5, 6, NULL, NULL, NULL, NULL, 7
        //            ])),
        //            1
        //        );
    }
}
