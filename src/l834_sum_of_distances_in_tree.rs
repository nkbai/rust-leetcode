/*
834. 树中距离之和

给定一个无向、连通的树。树中有 N 个标记为 0...N-1 的节点以及 N-1 条边 。

第 i 条边连接节点 edges[i][0] 和 edges[i][1] 。

返回一个表示节点 i 与其他所有节点距离之和的列表 ans。

示例 1:

输入: N = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
输出: [8,12,6,10,10,10]
解释:
如下为给定的树的示意图：
  0
 / \
1   2
   /|\
  3 4 5

我们可以计算出 dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
也就是 1 + 1 + 2 + 2 + 2 = 8。 因此，answer[0] = 8，以此类推。
说明: 1 <= N <= 10000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sum-of-distances-in-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
自底向上,
定义children:
如果节点a是节点n的直接子节点或者孙子节点,那么认为a都是n的children,n认为是n的children
每次返回两个信息:
1. [(children1到当前节点的距离,子节点1到所有其他节点距离之和(包含当前节点)]
如何合并:
针对如下树
```
       0
      /  \
     1    2
    /      \
   5        3
```
那么针对节点0
左侧子树返回为[(0,1),(1,1)],右侧子树返回为([(0,1),(1,1)]
那么0到当前节点为0,到所有其他节点之和为1+len(left)*1+1+len(right)*1
1到当前节点为0+1,到所有其他节点之和为1(自己到左子树节点距离之和)+len(right)*(0+2)+1(右子树2到所有其他节点距离之和)+1(到0的距离)
5到当前节点距离为1+1,到所有其他节点之和为1+len(right)*(1+2)+1+(1+1)
*/

struct Solution {}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    val: i32,
    nodes: Vec<Node>,
}
impl Solution {
    #[allow(dead_code)]
    pub fn sum_of_distances_in_tree(_n: i32, _edges: Vec<Vec<i32>>) -> Vec<i32> {
        Vec::new()
    }
    //todo 如何构建呢?不用unsafe
    fn build_tree_from_edges(mut edges: Vec<Vec<i32>>) -> Option<Node> {
        if edges.len() == 0 {
            return None;
        }
        let last_val = edges[0][0];
        let mut root = Node {
            val: last_val,
            nodes: Vec::new(),
        };
        let last_node = &mut root;
        while edges.len() > 0 {
            while edges.len() > 0 && edges[0][0] == last_val {
                last_node.nodes.push(Node {
                    val: edges[0][1],
                    nodes: Vec::new(),
                });
                edges.remove(0);
            }
            if edges.len() > 0 {
                let pos = last_node.nodes.iter().position(|n| n.val == last_val);
                if pos.is_none() {
                    panic!("not found");
                }
            }
        }
        Some(root)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_find_duplcates_tree() {
        //        [[0,1],[0,2],[2,3],[2,4],[2,5]]
        let t = Solution::build_tree_from_edges(vec![
            vec![0, 1],
            vec![0, 2],
            vec![2, 3],
            vec![2, 4],
            vec![2, 5],
        ]);
        println!("t={:?}", t)
    }
}
