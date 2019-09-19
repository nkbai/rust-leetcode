/*
101. 对称二叉树
给定一个二叉树，检查它是否是镜像对称的。

例如，二叉树 [1,2,2,3,4,4,3] 是对称的。

    1
   / \
  2   2
 / \ / \
3  4 4  3
但是下面这个 [1,2,2,null,3,null,3] 则不是镜像对称的:

    1
   / \
  2   2
   \   \
   3    3
说明:

如果你可以运用递归和迭代两种方法解决这个问题，会很加分。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/symmetric-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


*/

/*
思路:
相比之下,迭代更容易一些,
按层遍历,然后要求每层都是对称的即可.
因为可能缺节点,那么相应位置的值步特殊值NULL(-1),但是相应的节点不补
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    //判断一个数组是否是对称的
    fn is_symmetric_vec(v: &Vec<i32>) -> bool {
        if v.len() % 2 != 0 {
            return false;
        }
        for i in 0..v.len() {
            if v[i] != v[v.len() - 1 - i] {
                return false;
            }
        }
        return true;
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut h = Vec::new();
        if root.is_none() {
            return true;
        }
        //        println!("root={:?}", root);
        let mut v = Vec::new();
        v.push(root.unwrap().clone());
        h.push(v);
        while !h.is_empty() {
            let mut vr = Vec::new();
            let vh = h.remove(0); //移除第一个
                                  //            println!("vh={:?}", vh);
            let mut vh2 = Vec::new();
            vh.iter().for_each(|t| {
                //每次都是处理下一层是否对称,当前层能走到,说明已经是对称的了.
                if let Some(l) = t.borrow().left.clone() {
                    vr.push(l.borrow().val);
                    vh2.push(l);
                } else {
                    vr.push(-1);
                }
                if let Some(r) = t.borrow().right.clone() {
                    vr.push(r.borrow().val);
                    vh2.push(r)
                } else {
                    vr.push(-1);
                }
            });
            if !vh2.is_empty() {
                h.push(vh2);
            }
            if !Solution::is_symmetric_vec(&vr) {
                return false;
            }
        }
        return true;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_num_trees() {
        let r = build_tree(&vec![1, 2, 2, 3, 4, 4, 3]);
        assert_eq!(Solution::is_symmetric(r), true);
        let r = build_tree(&vec![1, 2, 2, NULL, 3, NULL, 3]);
        assert_eq!(Solution::is_symmetric(r), false);
    }
}
