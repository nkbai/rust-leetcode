#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
685. 冗余连接 II
在本问题中，有根树指满足以下条件的有向图。该树只有一个根节点，所有其他节点都是该根节点的后继。每一个节点只有一个父节点，除了根节点没有父节点。

输入一个有向图，该图由一个有着N个节点 (节点值不重复1, 2, ..., N) 的树及一条附加的边构成。附加的边的两个顶点包含在1到N中间，这条附加的边不属于树中已存在的边。

结果图是一个以边组成的二维数组。 每一个边 的元素是一对 [u, v]，用以表示有向图中连接顶点 u and v和顶点的边，其中父节点u是子节点v的一个父节点。

返回一条能删除的边，使得剩下的图是有N个节点的有根树。若有多个答案，返回最后出现在给定二维数组的答案。

示例 1:

输入: [[1,2], [1,3], [2,3]]
输出: [2,3]
解释: 给定的有向图如下:
  1
 / \
v   v
2-->3
示例 2:

输入: [[1,2], [2,3], [3,4], [4,1], [1,5]]
输出: [4,1]
解释: 给定的有向图如下:
5 <- 1 -> 2
     ^    |
     |    v
     4 <- 3
注意:

二维数组大小的在3到1000范围内。
二维数组中的每个整数在1到N之间，其中 N 是二维数组的大小。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/redundant-connection-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

 */

/*
思路:

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
        self.pre[prey as usize] = x;
        return true;
    }
}
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
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
        let t = Solution::find_redundant_directed_connection(edges);
        assert_eq!(t, vec![2, 3]);
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]];
        let t = Solution::find_redundant_directed_connection(edges);
        assert_eq!(t, vec![4, 1]);
    }
}
