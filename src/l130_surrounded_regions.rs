/*
130. 被围绕的区域

给定一个二维的矩阵，包含 'X' 和 'O'（字母 O）。

找到所有被 'X' 围绕的区域，并将这些区域里所有的 'O' 用 'X' 填充。

示例:

X X X X
X O O X
X X O X
X O X X
运行你的函数后，矩阵变为：

X X X X
X X X X
X X X X
X O X X
解释:

被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。 任何不在边界上，或不与边界上的 'O' 相连的 'O' 最终都会被填充为 'X'。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/surrounded-regions
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

/*
思路:
染色法:
凡是在边界上的o都会被染成红色,然后从他开始向所有邻居的o染色,
如果碰到已经染成红色的o,则跳过
最后扫描一遍,哪些没有被染色的都设置成x

时间,空间复杂度为O(N)
染色的时候最多访问两边.
最后替换的时候再走一遍

优化部分:
可以省掉空间复杂度O(N),碰到边界上的O用#来代替,最后所有的#用O代替,所有的O改为X
*/

struct Solution {}
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() == 0 || board[0].len() == 0 {
            return;
        }
        let mut mark = vec![vec![0; board[0].len()]; board.len()];
        //0 表示不确定 1表示已经染成红色了
        let row = board.len();
        let col = board[0].len();
        for i in 0..row {
            for j in 0..col {
                //是O,并且没有被染色,并且是在边界上
                if board[i][j] == 'O'
                    && mark[i][j] == 0
                    && (i == 0 || i == row - 1 || j == 0 || j == col - 1)
                {
                    Self::color((i, j), &mut mark, board);
                }
            }
        }
        for i in 0..row {
            for j in 0..col {
                //那些没有被染色的,肯定是被包围着的
                if board[i][j] == 'O' && mark[i][j] == 0 {
                    board[i][j] = 'X';
                }
            }
        }
    }
    fn color(pos: (usize, usize), mark: &mut Vec<Vec<i32>>, board: &Vec<Vec<char>>) {
        mark[pos.0][pos.1] = 1;
        let row = board.len();
        let col = board[0].len();
        let nexts = Self::get_next(pos, row, col);
        for next in nexts {
            if board[next.0][next.1] == 'O' && mark[next.0][next.1] == 0 {
                Self::color(next, mark, board);
            }
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

    #[test]
    fn test() {
        let mut v = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut v);
        println!("v={:?}", v);
    }
}
