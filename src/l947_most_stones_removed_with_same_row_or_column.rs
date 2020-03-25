#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
947. 移除最多的同行或同列石头

在二维平面上，我们将石头放置在一些整数坐标点上。每个坐标点上最多只能有一块石头。

现在，move 操作将会移除与网格上的某一块石头共享一列或一行的一块石头。

我们最多能执行多少次 move 操作？



示例 1：

输入：stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
输出：5
示例 2：

输入：stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
输出：3
示例 3：

输入：stones = [[0,0]]
输出：0


提示：

1 <= stones.length <= 1000
0 <= stones[i][j] < 10000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/most-stones-removed-with-same-row-or-column
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
首先解释一下题目,还是看了别人的解释才明白的
题目的意思是:
如果要移除一块石头,则必须有其他石头与其同一行或者同一列.
那么最多可以移除多少块石头.

如果两块石头在同一行或者同一列,则在一个并查集中.
先按行遍历,进行merge
然后按列遍历,进行merge
最后逐个统计

一个小技巧,将[2,2]转换为2*2+2 就可以直接使用并查集了.
*/
struct Solution {}
struct DSU {
    pre: Vec<i32>,
}
impl DSU {
    fn new(n: usize) -> DSU {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(i as i32);
        }
        DSU { pre: v }
    }
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
        self.pre[prey as usize] = prex;
        return true;
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let r = 100;
        let c = 100;
        let mut v = vec![vec![0; c]; r];
        for s in stones {
            v[s[0] as usize][s[1] as usize] = 1;
        }
        let mut dsu = DSU::new(r * c);
        for i in 0..r {
            let mut first = -1;
            for j in 0..c {
                if v[i][j] == 1 {
                    if first >= 0 {
                        dsu.merge((i * c + j) as i32, first);
                    } else {
                        first = (i * c + j) as i32;
                    }
                }
            }
        }
        for j in 0..c {
            let mut first = -1;
            for i in 0..r {
                if v[i][j] == 1 {
                    if first >= 0 {
                        dsu.merge((i * c + j) as i32, first);
                    } else {
                        first = (i * c + j) as i32;
                    }
                }
            }
        }
        let mut count = vec![0; r * c];
        let mut remove_count = 0;
        for i in 0..r {
            for j in 0..c {
                let x = dsu.find((i * c + j) as i32) as usize;
                count[x] += 1;
                if count[x] > 1 {
                    remove_count += 1;
                }
            }
        }
        remove_count
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ]);
        assert_eq!(t, 5);
        let t = Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 2],
            vec![1, 1],
            vec![2, 0],
            vec![2, 2],
        ]);
        assert_eq!(t, 3);
        let t = Solution::remove_stones(vec![vec![0, 0]]);
        assert_eq!(t, 0);
    }
}
