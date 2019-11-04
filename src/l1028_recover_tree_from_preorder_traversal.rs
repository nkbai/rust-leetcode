/*

1028. 从先序遍历还原二叉树

我们从二叉树的根节点 root 开始进行深度优先搜索。

在遍历中的每个节点处，我们输出 D 条短划线（其中 D 是该节点的深度），然后输出该节点的值。（如果节点的深度为 D，则其直接子节点的深度为 D + 1。根节点的深度为 0）。

如果节点只有一个子节点，那么保证该子节点为左子节点。

给出遍历输出 S，还原树并返回其根节点 root。



示例 1：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/recover-a-tree-from-preorder-traversal.png)

输入："1-2--3--4-5--6--7"
输出：[1,2,5,3,4,6,7]
示例 2：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/screen-shot-2019-04-10-at-114101-pm.png)

输入："1-2--3---4-5--6---7"
输出：[1,2,5,3,null,6,null,4,null,7]
示例 3：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/screen-shot-2019-04-10-at-114955-pm.png)

输入："1-401--349---90--88"
输出：[1,401,null,349,88,90]


提示：

原始树中的节点数介于 1 和 1000 之间。
每个节点的值介于 1 和 10 ^ 9 之间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/recover-a-tree-from-preorder-traversal
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
1. 递归处理
参数 1. 当前节点, 2 当前深度 3 剩下的待处理字节序列
2. nextDepth 获取下一个节点的深度
3. 如果 下一个深度和当前深度一样,则不处理,返回
4. 如果下一个深度是当前深度+1,如果没有左子节点则挂在左子树上,否则挂在右子树上
*/

use crate::share::TreeNode;
use std::cell::RefCell;

use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let (left, val) = Self::nextVal(s.as_bytes());
        let root = Rc::new(RefCell::new(TreeNode {
            left: None,
            right: None,
            val,
        }));
        Self::recover_internal(root.clone(), 0, left);
        Some(root)
    }
    fn recover_internal(root: Rc<RefCell<TreeNode>>, current_depth: usize, left: &[u8]) -> &[u8] {
        if left.len() == 0 {
            return left;
        }
        let (new_left, next_depth) = Self::nextDepth(left);
        if next_depth == current_depth + 1 {
            //构建左子树
            //取得下一个数字
            let (mut new_left, val) = Solution::nextVal(new_left);
            let n = Rc::new(RefCell::new(TreeNode {
                left: None,
                right: None,
                val,
            }));
            if root.borrow().left.is_none() {
                root.borrow_mut().left = Some(n.clone());
                new_left = Self::recover_internal(n, current_depth + 1, new_left);
                //试试剩下的是否是我的右子树,也可能是父节点的
                return Self::recover_internal(root, current_depth, new_left);
            } else {
                root.borrow_mut().right = Some(n.clone());
                ////已经是右子树了,剩下的要么是父节点的,要么是我的右子节点的
                return Self::recover_internal(n, current_depth + 1, new_left);
            }
        } else {
            //深度和当前一样,说明是父节点的右子节点
            return left;
        }
        left
    }
    fn nextDepth(left: &[u8]) -> (&[u8], usize) {
        if left[0] != '-' as u8 {
            panic!("first must b -");
        }
        let mut cnt = 0;

        for i in 0..left.len() {
            if left[i] == '-' as u8 {
                cnt = i + 1;
            } else {
                break;
            }
        }
        let (_, left) = left.split_at(cnt);
        return (left, cnt);
    }
    fn nextVal(left: &[u8]) -> (&[u8], i32) {
        if left[0] == '-' as u8 {
            panic!("first must be valid number");
        }
        let mut cnt = 0;
        for i in 0..left.len() {
            if left[i] != '-' as u8 {
                cnt = i + 1;
            } else {
                break;
            }
        }
        let (val_str, left) = left.split_at(cnt);
        use std::str;
        let s = str::from_utf8(val_str).expect("must be valid");
        let val: i32 = s.parse().unwrap();
        return (left, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let r = Solution::recover_from_preorder(String::from("1-2--3--4-5--6--7"));
        let r2 = build_tree_ignore_parent(&vec![1, 2, 5, 3, 4, 6, 7]);
        //        println!("r={}", r.unwrap().borrow().to_string());
        assert_eq!(r, r2);
        let r = Solution::recover_from_preorder(String::from("1-2--3---4-5--6---7"));
        let r2 = build_tree_ignore_parent(&vec![1, 2, 5, 3, null, 6, null, 4, null, 7]);
        //        println!("r={}", r.unwrap().borrow().to_string());
        assert_eq!(r, r2);
        let r = Solution::recover_from_preorder(String::from("1-401--349---90--88"));
        let r2 = build_tree_ignore_parent(&vec![1, 401, null, 349, 88, 90]);
        //        println!("r={}", r.unwrap().borrow().to_string());
        assert_eq!(r, r2);
    }
}
