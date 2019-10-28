/*
889. 根据前序和后序遍历构造二叉树

返回与给定的前序和后序遍历匹配的任何二叉树。

pre 和 post 遍历中的值是不同的正整数。



示例：

输入：pre = [1,2,4,5,3,6,7], post = [4,5,2,6,7,3,1]
输出：[1,2,3,4,5,6,7]


提示：

1 <= pre.length == post.length <= 30
pre[] 和 post[] 都是 1, 2, ..., pre.length 的排列
每个输入保证至少有一个答案。如果有多个答案，可以返回其中一个。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
题目中明确说了有可能有多个答案,只要有一个就ok了
1. 前序第一个当前子树的根节点,后序最后一个同样是当前子树的根节点,移除他们
2. 前序第一个是当前子树的左子节点(未必一定,但是行得通)
3. 前序第一个把后序分成两部分前半部分是左子树,后半部分是右子树,后半部分要移除最后一个
4. 左右两部分继续分别从第一步开始递归构造子树
*/
use crate::share::TreeNode;
use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::construct_internal(pre.as_slice(), post.as_slice());
    }
    fn construct_internal(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if pre.len() != post.len() {
            panic!("pre={:?},post={:?}", pre, post);
        }
        if pre.len() == 0 {
            return None;
        }
        let mut r = TreeNode::new(pre[0]); //第一个
        let l = pre.len();
        if l == 1 {
            return Some(Rc::new(RefCell::new(r))); //最后一个元素了,不用继续构建了
        }
        let m = pre[1];
        //必定找得到,这个位置就是左半部分的长度了
        let pos = post
            .iter()
            .position(|n| m == *n)
            .expect(format!("pre post 数据不一致,pre={:?},post={:?}", pre, post).as_str());
        //根据长度切割前序
        let (pre_left_first, pre_right) = pre.split_at(pos + 2); //左半部分多包含一个根节点
        let (_, pre_left) = pre_left_first.split_at(1); //左边至少包含两元素
                                                        //根据长度切割后序
        let (post_left, post_right_and_last) = post.split_at(pos + 1); //只是左半部分
        let (post_right, _) = post_right_and_last.split_at(post_right_and_last.len() - 1);
        r.left = Self::construct_internal(pre_left, post_left);
        r.right = Self::construct_internal(pre_right, post_right);
        return return Some(Rc::new(RefCell::new(r)));
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_construct_tree() {
        let pre = vec![1, 2, 4, 5, 3, 6, 7];
        let post = vec![4, 5, 2, 6, 7, 3, 1];
        println!(
            "tree={}",
            Solution::construct_from_pre_post(pre, post)
                .unwrap()
                .borrow()
                .to_string(),
        )
    }
}
