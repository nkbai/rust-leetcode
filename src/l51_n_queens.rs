/*
51. N皇后
n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。



上图为 8 皇后问题的一种解法。

给定一个整数 n，返回所有不同的 n 皇后问题的解决方案。

每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。

示例:

输入: 4
输出: [
 [".Q..",  // 解法 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // 解法 2
  "Q...",
  "...Q",
  ".Q.."]
]
解释: 4 皇后问题存在两个不同的解法。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/n-queens
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
典型的递归回溯问题,暴力穷举就ok了
*/
struct Solution {}
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut matrix = Vec::new();
        let mut col = Vec::new();
        for i in 0..n {
            let mut v = Vec::new();
            col.push(false);
            for j in 0..n {
                v.push(0);
            }
            matrix.push(v);
        }
        let mut result = Vec::new();

        Self::put(0, n as usize, &mut matrix, &mut col, &mut result);
        result
    }
    fn check_diagonal(row: usize, col: usize, n: usize, matrix: &Vec<Vec<i32>>) -> bool {
        //向右上检查对角线
        let mut r = row;
        let mut c = col;

        loop {
            if matrix[r][c] == 1 {
                return true;
            }
            if r == 0 || c == n - 1 {
                break;
            }
            r -= 1;
            c += 1;
        }

        //想左下检查对角线
        let mut r = row;
        let mut c = col;
        let mut found = false;
        loop {
            if matrix[r][c] == 1 {
                return true;
            }
            if r == n - 1 || c == 0 {
                break;
            }
            r += 1;
            c -= 1;
        }
        //左上检查对角线
        let mut r = row;
        let mut c = col;
        let mut found = false;
        loop {
            if matrix[r][c] == 1 {
                return true;
            }
            if r == 0 || c == 0 {
                break;
            }
            r -= 1;
            c -= 1;
        }
        //右下检查对角线
        let mut r = row;
        let mut c = col;
        let mut found = false;
        loop {
            if matrix[r][c] == 1 {
                return true;
            }
            if r == n - 1 || c == n - 1 {
                break;
            }
            r += 1;
            c += 1;
        }
        return false;
    }
    fn put(
        row: usize,
        n: usize,
        matrix: &mut Vec<Vec<i32>>,
        col: &mut Vec<bool>,
        result: &mut Vec<Vec<String>>,
    ) {
        if row == matrix.len() {
            result.push(Self::matrix_to_string(matrix));
            return;
        }
        for i in 0..n {
            if col[i] {
                continue; //这一列已经被占用了
            }
            if Self::check_diagonal(row, i, n, matrix) {
                continue;
            }
            //合格
            matrix[row][i] = 1;
            col[i] = true;
            Self::put(row + 1, n, matrix, col, result);
            //复原,尝试下一个位置
            matrix[row][i] = 0;
            col[i] = false;
        }
    }
    fn matrix_to_string(matrix: &Vec<Vec<i32>>) -> Vec<String> {
        let mut vec = Vec::new();
        for v in matrix {
            let mut bytes = Vec::new();
            for v2 in v {
                if *v2 == 1 {
                    bytes.push('Q' as u8);
                } else {
                    bytes.push('.' as u8);
                }
            }
            vec.push(unsafe { String::from_utf8_unchecked(bytes) });
        }
        vec
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let t = Solution::solve_n_queens(4);
        for i in t {
            println!("-----");
            for j in i {
                println!("{}", j);
            }
        }
    }
}
