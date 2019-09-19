/*
124. 二叉树中的最大路径和
给定一个非空二叉树，返回其最大路径和。

本题中，路径被定义为一条从树中任意节点出发，达到任意节点的序列。该路径至少包含一个节点，且不一定经过根节点。

示例 1:
```
输入: [1,2,3]

       1
      / \
     2   3

输出: 6
示例 2:

输入: [-10,9,20,null,null,15,7]

   -10
   / \
  9  20
    /  \
   15   7

输出: 42
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-maximum-path-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
不能使用贪心算法:
因为
```
    4
   /
  11
 /  \
7    2
```
这种情况,如果只看11,7,2这棵子树,那么最大路径和应该是20,也就是[7,11,2]这条路径
但是如果要看整体,则无疑[7,11,4]这条路径才是最大的,
那么针对这种情况,在11节点这个子树上来看,毫无疑问最大路径和是20,但是要提供两个选择,
一个是可以继续走下去的最大路径18,不能继续走下去的最大路径和20,

针对经过任意节点的最大路径,不外乎以下四种情况:
1. 只经过当前节点
2. 经过当前节点+左子树
3. 经过当前节点+右子树
4. 经过当前节点+左右子树
我们要同时返回任意节点的最大路径和总体最大路径
注意,因为负值的存在,所以不能认为最小路径是0
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let r = Solution::internal(root);
        if let Some(r) = r {
            return r.1;
        }
        return 0;
    }
    /*
    第一个整数: 经过当前节点的最大路径和,可以继续走下去
    第二个整数:整棵树的最大路径和,不能再继续走下去了
    */
    fn internal(root: Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32)> {
        if root.is_none() {
            return None;
        }
        let r = root.unwrap();
        let rv = r.borrow().val;
        let mut total_max = rv;
        let mut cur_max = rv;

        /*
        叶节点,就是他自己了
        */
        if r.borrow().left.is_none() && r.borrow().right.is_none() {
            return Some((r.borrow().val, r.borrow().val));
        }
        let lx = Solution::internal(r.borrow().left.clone());
        let rx = Solution::internal(r.borrow().right.clone());
        match (lx, rx) {
            (Some(l), Some(r)) => {
                /*
                左右子树都有
                先求出经过自己的最大路径和(只能左右选择一个)
                */
                let mut w_total_max; //经过我自己和两颗子树的尽可能大的那种情况
                if cur_max >= 0 {
                    w_total_max = cur_max;
                    if l.0 >= 0 {
                        w_total_max += l.0;
                    }
                    if r.0 >= 0 {
                        w_total_max += r.0;
                    }
                } else {
                    /*
                    自身小于0,那么经过我自己和两棵子树的所有可能最大值,
                    要区分两种情况:
                    1. 经过自己
                    2. 不经过自己
                    */

                    let not_contain_self = max(l.0, r.0);
                    let mut contain_self = rv;
                    if l.0 >= 0 {
                        contain_self += l.0;
                    }
                    if r.0 >= 0 {
                        contain_self += r.0;
                    }
                    w_total_max = max(not_contain_self, contain_self);
                }

                //经过自身,可以继续走下去的那条路径
                let lr_max = max(l.0, r.0);
                if lr_max >= 0 {
                    cur_max += lr_max;
                }
                /*
                到root为止,最大可能的路径和,不关心是否经过root,
                l.1,r.1都是不能继续走下去的路径
                */
                total_max = max(l.1, r.1);
                total_max = max(total_max, w_total_max);
            }
            //只有左子树,道理同上
            (Some(l), None) => {
                if l.0 >= 0 {
                    cur_max += l.0;
                }
                total_max = l.1;
                if cur_max > total_max {
                    total_max = cur_max;
                }
            }
            //只有右子树,道理同上
            (None, Some(r)) => {
                if r.0 >= 0 {
                    cur_max += r.0;
                }
                total_max = r.1;
                if cur_max > total_max {
                    total_max = cur_max;
                }
            }
            //没有子树,那么所有节点最大值和当前节点最大值就是他自身
            (None, None) => {}
        }
        println!(
            "r={:?},cur_max={},total_max={}",
            r.borrow().val,
            cur_max,
            total_max
        );
        return Some((cur_max, total_max));
    }

    pub fn max_path_sum2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = -9999999;
        Self::traverse(root, &mut |x: i32| {
            max = max.max(x);
        });
        println!("{:?}", max);
        max
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, f: &mut impl FnMut(i32)) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut r_b = root.as_ref().unwrap().borrow_mut();
        let (l, r) = (r_b.left.take(), r_b.right.take());
        let (lval, rval) = (Self::traverse(l, f).max(0), Self::traverse(r, f).max(0));
        let cur_path_max = lval + rval + r_b.val;
        f(cur_path_max);
        (lval + r_b.val).max(rval + r_b.val)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::build_tree;
    use crate::share::NULL;
    #[test]
    fn test_has_path_sum() {
        /*
                [
           [5,4,11,2],
           [5,8,4,5]
        ]
                */
        let t = build_tree(&vec![1, 2, 3]);
        println!("t={:?}", t);
        assert_eq!(6, Solution::max_path_sum(t));
        let t = build_tree(&vec![-10, 9, 20, NULL, NULL, 15, 7]);
        println!("t={:?}", t);
        assert_eq!(42, Solution::max_path_sum(t));

        let t = build_tree(&vec![5, 4, 8, 11, NULL, 13, 4, 7, 2, NULL, NULL, NULL, 1]);
        println!("t={:?}", t);
        assert_eq!(49, Solution::max_path_sum(t));

        let t = build_tree(&vec![
            5, 4, 8, 11, NULL, 13, 4, 7, 2, NULL, NULL, NULL, NULL, NULL, 1,
        ]);
        println!("t={:?}", t);
        assert_eq!(48, Solution::max_path_sum(t));
        let t = TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -9,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: None,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    right: None,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: -9,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: -2,
                                left: None,
                                right: None,
                            }))),
                        }))),
                    }))),
                }))),
            }))),
        };
        let t = Some(Rc::new(RefCell::new(t)));
        println!("t={:?}", t);
        assert_eq!(9, Solution::max_path_sum2(t));
    }
}
