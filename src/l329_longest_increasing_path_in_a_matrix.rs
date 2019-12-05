/*
329. 矩阵中的最长递增路径
给定一个整数矩阵，找出最长递增路径的长度。

对于每个单元格，你可以往上，下，左，右四个方向移动。 你不能在对角线方向上移动或移动到边界外（即不允许环绕）。

示例 1:

输入: nums =
[
  [9,9,4],
  [6,6,8],
  [2,1,1]
]
输出: 4
解释: 最长递增路径为 [1, 2, 6, 9]。
示例 2:

输入: nums =
[
  [3,4,5],
  [3,2,6],
  [2,2,1]
]
输出: 4
解释: 最长递增路径是 [3, 4, 5, 6]。注意不允许在对角线方向上移动。
*/
/*
思路:贪心算法
从0开始尽可能的往前走,
会找到一条最长的路径,同时这也意味着这条路径上的每个节点的最长路径都找到了.
记下来.
然后尝试下一个,如果碰到了直接使用历史值
每个节点最多访问一遍
因此复杂度是O(N)
*/
struct Solution {}
use std::cmp::max;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }
        let mut mark = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut max_path = 0;
        let row_count = matrix.len();
        let col_count = matrix[0].len();
        for i in 0..row_count {
            for j in 0..col_count {
                if mark[i][j] > 0 {
                    continue; //尝试过了,不用再尝试了.
                }
                let p = Self::visit(&matrix, i, j, &mut mark);
                if p > max_path {
                    max_path = p;
                }
            }
        }
        max_path
    }
    fn visit(matrix: &Vec<Vec<i32>>, i: usize, j: usize, mark: &mut Vec<Vec<i32>>) -> i32 {
        let c = matrix[i][j];
        if mark[i][j] > 0 {
            return mark[i][j];
        }
        let nexts = Self::get_next((i, j), matrix.len(), matrix[0].len());
        let mut p = 1;
        for next in nexts {
            if matrix[next.0][next.1] <= c {
                //一定不会走回头路,因为这违反了递增的规则
                continue;
            }
            let p1 = Self::visit(matrix, next.0, next.1, mark);
            p = max(p1 + 1, p);
        }
        mark[i][j] = p;
        return p;
    }
    fn get_next(pos: (usize, usize), row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        if pos.1 + 1 < col {
            v.push((pos.0, pos.1 + 1));
        }
        if pos.1 >= 1 {
            v.push((pos.0, pos.1 - 1));
        }
        if pos.0 + 1 < row {
            v.push((pos.0 + 1, pos.1));
        }
        if pos.0 >= 1 {
            v.push((pos.0 - 1, pos.1));
        }
        v
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let t =
            Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]);
        assert_eq!(t, 4);
        let t = Solution::longest_increasing_path(vec![vec![0], vec![1], vec![5], vec![5]]);
        assert_eq!(t, 3);
    }
}
