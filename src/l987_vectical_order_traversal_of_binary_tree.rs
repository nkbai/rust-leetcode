/*
987. 二叉树的垂序遍历


给定二叉树，按垂序遍历返回其结点值。

对位于 (X, Y) 的每个结点而言，其左右子结点分别位于 (X-1, Y-1) 和 (X+1, Y-1)。

把一条垂线从 X = -infinity 移动到 X = +infinity ，每当该垂线与结点接触时，我们按从上到下的顺序报告结点的值（ Y 坐标递减）。

如果两个结点位置相同，则首先报告的结点值较小。

按 X 坐标顺序返回非空报告的列表。每个报告都有一个结点值列表。



示例 1：
```viz
digraph G {
    node [shape=circle]
    edge [arrowhead=vee]
   3-> 9
   3-> 20
   20->15
   20->7
}
```


输入：[3,9,20,null,null,15,7]
输出：[[9],[3,15],[20],[7]]
解释：
在不丧失其普遍性的情况下，我们可以假设根结点位于 (0, 0)：
然后，值为 9 的结点出现在 (-1, -1)；
值为 3 和 15 的两个结点分别出现在 (0, 0) 和 (0, -2)；
值为 20 的结点出现在 (1, -1)；
值为 7 的结点出现在 (2, -2)。
示例 2：
```viz
digraph G {
    node [shape=circle]
    edge [arrowhead=vee]
   1-> 2
   1-> 3
   2->4
   2->5
   3->6
   3->7
}
```


输入：[1,2,3,4,5,6,7]
输出：[[4],[2],[1,5,6],[3],[7]]
解释：
根据给定的方案，值为 5 和 6 的两个结点出现在同一位置。
然而，在报告 "[1,5,6]" 中，结点值 5 排在前面，因为 5 小于 6。


提示：

树的结点数介于 1 和 1000 之间。
每个结点值介于 0 和 1000 之间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/vertical-order-traversal-of-a-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
1. 按照深度优先遍历,
2. 使用一个Btreemap来保存垂直层次关系 key: X坐标,value: map<Y,vec<i32>>
3. 针对X,Y坐标相同的位置,排序后返回结果
*/
use crate::share::TreeNode;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut m = BTreeMap::new();
        Solution::vertical_traversal_internal(root, &mut m, 0, 0);
        let mut v = Vec::new();
        m.iter_mut().for_each(|m2| {
            let mut v2 = Vec::new();
            println!("x={},m={:?}", m2.0, m2.1);
            m2.1.iter_mut().for_each(|v3| {
                v3.1.sort();
                v2.extend_from_slice(v3.1.as_slice());
            });
            v.push(v2);
        });
        v
    }
    fn vertical_traversal_internal(
        root: Option<Rc<RefCell<TreeNode>>>,
        m: &mut BTreeMap<i32, BTreeMap<i32, Vec<i32>>>,
        x: i32,
        y: i32,
    ) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap();
        if let Some(m2) = m.get_mut(&x) {
            //x已经存在,y也存在,完全相同的位置,到时候需要排序的
            if let Some(v) = m2.get_mut(&y) {
                v.push(r.borrow().val)
            } else {
                //x相同,但是y还不存在
                m2.insert(y, vec![r.borrow().val]);
            }
        } else {
            //初次出现的X坐标
            let mut m2 = BTreeMap::new();
            m2.insert(y, vec![r.borrow().val]);
            m.insert(x, m2);
        }
        /*
            由于BTreeMap key排序比较麻烦,这里简单让y坐标向上增长,效果完全相同
        */
        Solution::vertical_traversal_internal(r.borrow().left.clone(), m, x - 1, y + 1);
        Solution::vertical_traversal_internal(r.borrow().right.clone(), m, x + 1, y + 1);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test_find_duplcates_tree() {
        let t = build_tree_ignore_parent(&vec![3, 9, 20, NULL, NULL, 15, 7]);
        let r = Solution::vertical_traversal(t);
        assert_eq!(r, vec![vec![9], vec![3, 15], vec![20], vec![7]]);
        let t = build_tree_ignore_parent(&vec![1, 2, 3, 4, 5, 6, 7]);
        let r = Solution::vertical_traversal(t);
        assert_eq!(r, vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]]);
    }
}
