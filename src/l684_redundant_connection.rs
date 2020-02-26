#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
684. 冗余连接
在本问题中, 树指的是一个连通且无环的无向图。

输入一个图，该图由一个有着N个节点 (节点值不重复1, 2, ..., N) 的树及一条附加的边构成。附加的边的两个顶点包含在1到N中间，这条附加的边不属于树中已存在的边。

结果图是一个以边组成的二维数组。每一个边的元素是一对[u, v] ，满足 u < v，表示连接顶点u 和v的无向图的边。

返回一条可以删去的边，使得结果图是一个有着N个节点的树。如果有多个答案，则返回二维数组中最后出现的边。答案边 [u, v] 应满足相同的格式 u < v。

示例 1：

输入: [[1,2], [1,3], [2,3]]
输出: [2,3]
解释: 给定的无向图为:
  1
 / \
2 - 3
示例 2：

输入: [[1,2], [2,3], [3,4], [1,4], [1,5]]
输出: [1,4]
解释: 给定的无向图为:
5 - 1 - 2
    |   |
    4 - 3
注意:

输入的二维数组大小在 3 到 1000。
二维数组中的整数在1到N之间，其中N是输入数组的大小。
更新(2017-09-26):
我们已经重新检查了问题描述及测试用例，明确图是无向 图。对于有向图详见冗余连接II。对于造成任何不便，我们深感歉意。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/redundant-connection
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
统计每个节点的入度,然后按照入度排序
0的是孤立点
1的是树的定点
然后广度优先遍历,如果碰到遍历过的点二次遍历,说明该边是冗余的
考虑一个问题:
[1,4] 在这过程中我们要确定输出的是[1,4]而不是[4,1]

空间复杂度分析:
O(N) ,因为mutable的问题,中间使用了clone,可以使用unsafe绕开
时间复杂度:
因为边的个数和定点的个数是一致的,所以
是O(NLogN)
第一步建图是O(NLogN),最后一步遍历也是O(NLogN),要删除

此方法失败:
因为无法满足题目中的一条规则: 如果有多个答案,则返回二维数组中最后出现的边.
*/
struct Solution {}
use std::collections::HashSet;
#[derive(Default, Clone, Debug)]
struct NodeInfo {
    in_bounds: i32,
    children: HashSet<(i32, bool)>,
}
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        //多1是简化处理,忽略下标0
        let mut nodes: Vec<NodeInfo> = vec![Default::default(); edges.len() + 1];
        for e in edges.iter() {
            let n = nodes.get_mut(e[0] as usize).unwrap();
            n.in_bounds += 1;
            n.children.insert((e[1], false));
            let n2 = nodes.get_mut(e[1] as usize).unwrap();
            n2.in_bounds += 1;
            n2.children.insert((e[0], true));
        }
        // nodes.sort_by(|n1, n2| n1.in_bounds.cmp(n2.in_bounds));
        let mut set = vec![false; edges.len() + 1];
        let mut queue = vec![];
        for (pos, n) in nodes.iter().enumerate() {
            if n.in_bounds <= 1 {
                set[pos] = true;
                queue.push(pos as i32);
            }
        }
        let mut pos = 0;
        while pos < queue.len() {
            let v = queue[pos];
            let n = nodes.get_mut(v as usize).unwrap();
            println!("v={},n={:?}", v, n);
            for c in n.children.clone() {
                if set[c.0 as usize] == true {
                    if c.1 {
                        return vec![c.0, v];
                    } else {
                        return vec![v, c.0];
                    }
                } else {
                    set[c.0 as usize] = true;
                    queue.push(c.0);
                    //一条边不要重复计算
                    nodes[c.0 as usize].children.remove(&(v, !c.1));
                }
            }
            pos += 1;
        }
        panic!("must found")
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        // let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        // let t = Solution::find_redundant_connection(edges);
        // assert_eq!(t, vec![2, 3]);
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        let t = Solution::find_redundant_connection(edges);
        assert_eq!(t, vec![1, 4]);
    }
}
