#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
1319. 连通网络的操作次数
用以太网线缆将 n 台计算机连接成一个网络，计算机的编号从 0 到 n-1。线缆用 connections 表示，其中 connections[i] = [a, b] 连接了计算机 a 和 b。

网络中的任何一台计算机都可以通过网络直接或者间接访问同一个网络中其他任意一台计算机。

给你这个计算机网络的初始布线 connections，你可以拔开任意两台直连计算机之间的线缆，并用它连接一对未直连的计算机。请你计算并返回使所有计算机都连通所需的最少操作次数。如果不可能，则返回 -1 。



示例 1：
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/11/sample_1_1677.png)


输入：n = 4, connections = [[0,1],[0,2],[1,2]]
输出：1
解释：拔下计算机 1 和 2 之间的线缆，并将它插到计算机 1 和 3 上。
示例 2：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/11/sample_2_1677.png)

输入：n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
输出：2
示例 3：

输入：n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
输出：-1
解释：线缆数量不足。
示例 4：

输入：n = 5, connections = [[0,1],[0,2],[3,4],[2,3]]
输出：0


提示：

1 <= n <= 10^5
1 <= connections.length <= min(n*(n-1)/2, 10^5)
connections[i].length == 2
0 <= connections[i][0], connections[i][1] < n
connections[i][0] != connections[i][1]
没有重复的连接。
两台计算机不会通过多条线缆连接。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/number-of-operations-to-make-network-connected
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


 */

/*
思路:
并查集
假设一开始集合的总数为n,
每来一对,则集合数量要么减一要么就是网线富裕一根.
最后,如果富裕的网线数量大于(n-1)则返回n-1,否则返回-1
*/
struct Solution {}
struct DSU {
    pre: Vec<usize>,
}
impl DSU {
    pub fn find(&mut self, x: usize) -> usize {
        if self.pre[x] == x {
            return x;
        }
        let prex = self.find(self.pre[x]);
        self.pre[x] = prex;
        return prex;
    }
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let prex = self.find(x);
        let prey = self.find(y);
        if prex == prey {
            return false;
        }
        self.pre[prey] = prex;
        return true;
    }
}

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut dsu = DSU { pre: vec![] };
        for i in 0..n as usize {
            dsu.pre.push(i);
        }
        let mut n = n;
        let mut lines = 0;
        for c in connections {
            let a = c[0] as usize;
            let b = c[1] as usize;
            if dsu.merge(a, b) {
                n -= 1;
            } else {
                lines += 1;
            }
        }
        if lines >= n - 1 {
            return n - 1;
        } else {
            return -1;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
        assert_eq!(t, 1);
        let t = Solution::make_connected(
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]],
        );
        assert_eq!(t, 2);
        let t = Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]);
        assert_eq!(t, -1);
        let t = Solution::make_connected(5, vec![vec![0, 1], vec![0, 2], vec![3, 4], vec![2, 3]]);
        assert_eq!(t, 0);
    }
}
