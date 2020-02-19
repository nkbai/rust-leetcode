/*
863. 二叉树中所有距离为 K 的结点
给定一个二叉树（具有根结点 root）， 一个目标结点 target ，和一个整数值 K 。

返回到目标结点 target 距离为 K 的所有结点的值的列表。 答案可以以任何顺序返回。



示例 1：

输入：root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, K = 2

输出：[7,4,1]

解释：
所求结点为与目标结点（值为 5）距离为 2 的结点，
值分别为 7，4，以及 1

![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/28/sketch0.png)

注意，输入的 "root" 和 "target" 实际上是树上的结点。
上面的输入仅仅是对这些对象进行了序列化描述。


提示：

给定的树是非空的，且最多有 K 个结点。
树上的每个结点都具有唯一的值 0 <= node.val <= 500 。
目标结点 target 是树上的结点。
0 <= K <= 1000.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/all-nodes-distance-k-in-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
1. 前序遍历,找到target
2. 函数返回参数
+ 当前节点距离target的距离,
+ 是否已经找到target
每个节点先遍历自身,然后是左子树,然后是右子树
如果是在柚子树上找到了target,还需要再次遍历左子树

对于防止重复遍历,简单使用set来存储数据
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: i32, k: i32) -> Vec<i32> {
        let mut h = HashSet::new();
        Self::internal(root, target, k, -1, &mut h, &mut false);
        h.iter().map(|n| *n).collect()
    }
    fn internal(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        k: i32,
        this_distance: i32,
        v: &mut HashSet<i32>,
        found: &mut bool,
    ) -> i32 {
        if root.is_none() || this_distance > k {
            return -1;
        }
        let r = root.unwrap();
        println!("visit {}", r.borrow().val);
        //找到target后的逻辑就比较简单,所有节点都是加一的距离
        if *found {
            if this_distance == k {
                v.insert(r.borrow().val);
                println!(
                    "push {},distance={},found={}",
                    r.borrow().val,
                    this_distance,
                    found
                );
            }

            Self::internal(
                r.borrow().left.clone(),
                target,
                k,
                this_distance + 1,
                v,
                found,
            );
            Self::internal(
                r.borrow().right.clone(),
                target,
                k,
                this_distance + 1,
                v,
                found,
            );
            return this_distance;
        } else {
            //还没有找到target,正常遍历
            if r.borrow().val == target {
                //我就是要找的那个target
                *found = true;
                Self::internal(r.borrow().left.clone(), target, k, 1, v, found);
                Self::internal(r.borrow().right.clone(), target, k, 1, v, found);
                return 0;
            } else {
                //尝试遍历
                let d1 = Self::internal(r.borrow().left.clone(), target, k, -1, v, found);
                if d1 >= 0 && d1 <= k {
                    if d1 + 1 == k {
                        //自己可以放进去,然后尝试放右子树
                        v.insert(r.borrow().val);
                        println!(
                            "push {},distance={},found={}",
                            r.borrow().val,
                            this_distance,
                            found
                        );
                    }
                    Self::internal(r.borrow().right.clone(), target, k, d1 + 2, v, found);
                    return d1 + 1;
                } else {
                    //左子树没找到,遍历右子树
                    let d2 = Self::internal(r.borrow().right.clone(), target, k, -1, v, found);
                    if d2 >= 0 && d2 <= k {
                        if d2 + 1 == k {
                            //在右子树上找到了target,再次遍历左子树
                            v.insert(r.borrow().val);
                            println!(
                                "push {},distance={},found={}",
                                r.borrow().val,
                                this_distance,
                                found
                            );
                        }
                        Self::internal(r.borrow().left.clone(), target, k, d2 + 2, v, found);
                        return d2 + 1;
                    }
                    return -1; //左右子树都没有找到,那就继续找
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        let t = build_tree_ignore_parent(&vec![3, 5, 1, 6, 2, 0, 8, NULL, NULL, 7, 4]);
        let r = Solution::distance_k(t.clone(), 5, 2);
        let mut s = HashSet::new();
        s.insert(7);
        s.insert(4);
        s.insert(1);

        let s2: HashSet<i32> = r.iter().map(|n| *n).collect();
        assert_eq!(s, s2);

        let r = Solution::distance_k(t.clone(), 8, 2);
        let mut s = HashSet::new();
        s.insert(0);
        s.insert(3);

        let s2: HashSet<i32> = r.iter().map(|n| *n).collect();
        assert_eq!(s, s2);
    }
}
