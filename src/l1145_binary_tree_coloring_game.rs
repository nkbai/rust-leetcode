/*
1145. 二叉树着色游戏

有两位极客玩家参与了一场「二叉树着色」的游戏。游戏中，给出二叉树的根节点 root，树上总共有 n 个节点，且 n 为奇数，其中每个节点上的值从 1 到 n 各不相同。



游戏从「一号」玩家开始（「一号」玩家为红色，「二号」玩家为蓝色），最开始时，

「一号」玩家从 [1, n] 中取一个值 x（1 <= x <= n）；

「二号」玩家也从 [1, n] 中取一个值 y（1 <= y <= n）且 y != x。

「一号」玩家给值为 x 的节点染上红色，而「二号」玩家给值为 y 的节点染上蓝色。



之后两位玩家轮流进行操作，每一回合，玩家选择一个他之前涂好颜色的节点，将所选节点一个 未着色 的邻节点（即左右子节点、或父节点）进行染色。

如果当前玩家无法找到这样的节点来染色时，他的回合就会被跳过。

若两个玩家都没有可以染色的节点时，游戏结束。着色节点最多的那位玩家获得胜利 ✌️。



现在，假设你是「二号」玩家，根据所给出的输入，假如存在一个 y 值可以确保你赢得这场游戏，则返回 true；若无法获胜，就请返回 false。



示例：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/08/04/1480-binary-tree-coloring-game.png)
输入：root = [1,2,3,4,5,6,7,8,9,10,11], n = 11, x = 3
输出：True
解释：第二个玩家可以选择值为 2 的节点。


提示：

二叉树的根节点为 root，树上由 n 个节点，节点上的值从 1 到 n 各不相同。
n 为奇数。
1 <= x <= n <= 100

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-tree-coloring-game
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
1. 首先树是联通的,从任意一个节点都可以走遍整棵树的任意节点
2. 从任意固定节点抵达其他节点都只有一条路径,不存在多条路径的情况.
当选定x以后,树就分成了三部分,左子树,右子树和从父节点出发的路
那么就是找寻一下能否在x的这三个紧邻部分选一个y,让y覆盖的节点数多余剩下的两个之和.
*/

use crate::share::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    /*
    n:实际上就是树中节点的个数
    采用前序遍历,碰到x后,对x的左右子树分别计数,
    得到三个部分的个数即可
    只要有一个部分的个数超过另外两个之和,那就应该认为我可以赢.
    */
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let mut found = false;
        let (l, r) = Self::find_x(root, x, &mut found);
        if !found {
            //没有x
            return true;
        }
        let path1 = l;
        let path2 = r;
        let path3 = n - (l + r + 1);
        if path1 > path2 + path3 {
            return true;
        }
        if path2 > path1 + path3 {
            return true;
        }
        if path3 > path1 + path2 {
            return true;
        }
        return false;
    }
    //在树中查找x,并且返回x左右两颗子树的个数
    fn find_x(root: Option<Rc<RefCell<TreeNode>>>, x: i32, found: &mut bool) -> (i32, i32) {
        if root.is_none() {
            return (-1, -1);
        }
        let r = root.unwrap();
        if r.borrow().val == x {
            //找到了
            *found = true;
            let l = Self::count_nodes(r.borrow().left.clone());
            let r = Self::count_nodes(r.borrow().right.clone());
            return (l, r);
        } else {
            let l = Self::find_x(r.borrow().left.clone(), x, found);
            if *found {
                return l;
            }
            let r = Self::find_x(r.borrow().right.clone(), x, found);
            if *found {
                return r;
            }
        }
        return (-1, -1);
    }
    fn count_nodes(n: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if n.is_none() {
            return 0;
        }
        let n = n.unwrap();
        let l = Self::count_nodes(n.borrow().left.clone());
        let r = Self::count_nodes(n.borrow().right.clone());
        return l + r + 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let t = build_tree_ignore_parent(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 1), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 2), false);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 3), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 4), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 5), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 6), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 7), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 8), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 9), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 10), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 11), true);
        assert_eq!(Solution::btree_game_winning_move(t.clone(), 11, 12), true);
    }
}
