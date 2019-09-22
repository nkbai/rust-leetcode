/*
543. 二叉树的直径
给定一棵二叉树，你需要计算它的直径长度。一棵二叉树的直径长度是任意两个结点路径长度中的最大值。这条路径可能穿过根结点。

示例 :
给定二叉树

          1
         / \
        2   3
       /  \
      4    5
     /      \
    0        6
              \
               7
                \
                 8
返回 3, 它的长度是路径 [0,4,2,5,6,7,8] 或者 [3,1,2,5,6,7,8]。

注意：两结点之间的路径长度是以它们之间边的数目表示。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/diameter-of-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
每个节点都会返回三个值:
1. 经过自身(或者不经过自身)截至的最大直径长度,比如对于节点2,4-2-5 这条路径,最大节点数是3
2.   不经过自身的最大直径长度,如果上图0还有一个子节点9,那么针对节点1,返回值应该是
 经过自身最大直径长度是7,继续走下去的是6,整体最大长度则是8
  这种情况最大直径长度就是7
3. 经过自身可以继续走下去最大直径长度,比如对于节点2,只能是4-2


*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut h, _) = Self::internal(root);
        //刚刚算的是节点数,要减去1,还要考虑到树是空的情况,避免出现负值
        if h >= 1 {
            h -= 1;
        }
        return h;
    }
    /*
    第一个: 经过自身(或者不经过自身)截至的最大节点数
    第二个: 经过自身可以继续的最大节点数
    */
    fn internal(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let r = root.unwrap();
        let (l1, l2) = Self::internal(r.borrow().left.clone());
        let (r1, r2) = Self::internal(r.borrow().right.clone());
        let mut c2 = max(l2, r2) + 1;
        let mut c1 = l2 + r2 + 1;
        c1 = max(c1, l1);
        c1 = max(c1, r1);
        println!("node={},ret={:?}", r.borrow().val, (c1, c2));
        return (c1, c2);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_level_order_bottom() {
        //        assert_eq!(
        //            Solution::diameter_of_binary_tree(build_tree(&vec![1, 2, 3, 4, 5])),
        //            3
        //        );
        //        assert_eq!(
        //            Solution::diameter_of_binary_tree(build_tree(&vec![2, 1, 3])),
        //            2
        //        );
        let t = build_tree(&vec![
            4, //注释
            -7, -3, //换行
            NULL, NULL, -9, -3, //换行
            NULL, NULL, NULL, NULL, 9, -7, -4, NULL, //换行
            NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, 6, NULL, -6, -6, NULL, NULL, NULL,
            NULL, //换行
            NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
            NULL, NULL, 0, 6, NULL, NULL, 5, NULL, 9, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
            NULL, NULL, //huanhang
            NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
            NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
            NULL, NULL, NULL, NULL, //ZHONGJIAN
            NULL, -1, -4, NULL, NULL, NULL, NULL, NULL, -2, //最后一行
        ]);
        println!("t={:?}", t);
        assert_eq!(Solution::diameter_of_binary_tree(t), 8);
    }
}
