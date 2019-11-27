/*
64. 最小路径和
给定一个包含非负整数的 m x n 网格，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

说明：每次只能向下或者向右移动一步。

示例:

输入:
[
[1,3,1],
  [1,5,1],
  [4,2,1]
]
输出: 7
解释: 因为路径 1→3→1→1→1 的总和最小。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-path-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
自底向上
建立同样的mxn数组,里面保存的是每个点到右下角的最短路径
任意一个点前进方向只有两个
要么向右,要么向下
去两者较小的就是自身的最短路径了
*/

use std::cmp::min;
use std::iter::Rev;

struct Solution {}
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = Vec::new();
        if grid.is_empty() {
            return 0;
        }
        //初始化min,建立同样的mxn数组
        grid.iter().for_each(|_| {
            let mut v = Vec::new();
            grid[0].iter().for_each(|_| {
                v.push(0);
            });
            dp.push(v);
        });
        let r = grid.len();
        let c = grid[0].len();
        if c == 0 {
            return 0;
        }
        for i in (0..r).rev() {
            for j in (0..c).rev() {
                let mut right = std::i32::MAX; //向右走的
                let mut down = std::i32::MAX;
                if j + 1 < c {
                    right = dp[i][j + 1] + grid[i][j];
                }
                if i + 1 < r {
                    down = dp[i + 1][j] + grid[i][j];
                }
                if i != r - 1 || j != c - 1 {
                    let c = min(right, down);
                    dp[i][j] = c;
                    println!("({},{})=>{}", i, j, c);
                } else {
                    dp[i][j] = grid[i][j];
                }
            }
        }
        dp[0][0]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]);
        assert_eq!(t, 7);
    }
}
