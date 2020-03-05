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
并查集
如果新进来的边连接的两个节点已经在集合中,那么他就是冗余的.
按照顺序加入边,保证返回的是最后出现的边
空间复杂度:
O(N)
时间复杂度:
也是O(N),一条边最多走两边.

*/
struct Solution {}
struct DSU {
    pre: Vec<i32>,
}
impl DSU {
    fn find(&mut self, x: i32) -> i32 {
        if self.pre[x as usize] == x {
            return x;
        }
        // let prex = self.pre[x];
        let prex = self.find(self.pre[x as usize]);
        //因为递归,这里会把一串上面的所有路径都压缩了,
        self.pre[x as usize] = prex;
        return prex;
    }
    //返回false,说明x,y在同一个集合里
    fn merge(&mut self, x: i32, y: i32) -> bool {
        let prex = self.find(x);
        let prey = self.find(y);
        if prex == prey {
            return false;
        }
        //注意这里是设置的是prex的parent,而不是x的parent
        self.pre[prex as usize] = prey;
        return true;
    }
}
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = DSU { pre: vec![] };
        for i in 0..edges.len() + 1 {
            dsu.pre.push(i as i32);
        }
        //按照并查集的规则,把所有的边都放在一个集合中,因为只有一条冗余的边
        //如果重复放置,那就是它了.
        for e in edges.iter() {
            // println!("edge={:?}", e);
            if !dsu.merge(e[0], e[1]) {
                return e.clone();
            }
        }
        panic!()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let t = Solution::find_redundant_connection(edges);
        assert_eq!(t, vec![2, 3]);
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        let t = Solution::find_redundant_connection(edges);
        assert_eq!(t, vec![1, 4]);
    }
}
