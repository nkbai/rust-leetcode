/*
给定一个由 '1'（陆地）和 '0'（水）组成的的二维网格，计算岛屿的数量。一个岛被水包围，并且它是通过水平方向或垂直方向上相邻的陆地连接而成的。你可以假设网格的四个边均被水包围。

示例 1:

输入:
11110
11010
11000
00000

输出: 1
示例 2:

输入:
11000
11000
00100
00011

输出: 3
*/
struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }
        let mut visit = vec![vec![false; grid[0].len()]; grid.len()];
        let mut cnt = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' && !visit[i][j] {
                    Self::traversal(i, j, &grid, &mut visit);
                    cnt += 1;
                }
            }
        }
        cnt
    }
    //尽可能的访问并标记所有的1
    fn traversal(r: usize, c: usize, grid: &Vec<Vec<char>>, visit: &mut Vec<Vec<bool>>) {
        let total_row = grid.len();
        let total_col = grid[0].len();
        let mut q = Vec::new();
        q.push((r, c));
        while q.len() > 0 {
            let p = q.remove(0);
            if visit[p.0][p.1] {
                continue;
            }
            let next = Self::get_next(p, total_row, total_col);
            next.iter().for_each(|p| {
                if grid[p.0][p.1] == '1' && !visit[p.0][p.1] {
                    q.push((p.0, p.1));
                }
            });
            visit[p.0][p.1] = true;
        }
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
    use crate::share::*;
    #[test]
    fn test() {
        let v = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let t = Solution::num_islands(v);
        assert_eq!(t, 1);
        let v = vec![
            vec!['1', '1', '1'],
            vec!['0', '1', '0'],
            vec!['1', '1', '1'],
        ];
        let t = Solution::num_islands(v);
        assert_eq!(t, 1);
    }
}
