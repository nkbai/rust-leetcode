/*
79. 单词搜索
给定一个二维网格和一个单词，找出该单词是否存在于网格中。

单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。

示例:

board =
[
  ['A','B','C','E'],
  ['S','F','C','S'],
  ['A','D','E','E']
]

给定 word = "ABCCED", 返回 true.
给定 word = "SEE", 返回 true.
给定 word = "ABCB", 返回 false.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/word-search
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
最笨的办法就是挨个找,如果找到了第一个字符,那就按照图的遍历方式,一直遍历下去
直到走完整个string,否则回溯尝试其他路径
复杂度:?

*/
struct Solution {}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.len() == 0 {
            return word.len() == 0;
        }
        let row_count = board.len();
        let col_count = board[0].len();
        let mut visit = vec![vec![false; col_count]; row_count];
        let word = word.as_bytes();

        for i in 0..row_count {
            for j in 0..col_count {
                let r = Self::dfs(&board, word, i, j, &mut visit);
                if r {
                    return true;
                }
            }
        }
        false
    }
    fn dfs(
        board: &Vec<Vec<char>>,
        word: &[u8],
        i: usize,
        j: usize,
        visit: &mut Vec<Vec<bool>>,
    ) -> bool {
        if word.len() == 0 {
            return true;
        }
        if board[i][j] != word[0] as char {
            return false;
        }
        if visit[i][j] {
            return false;
        }
        if word.len() == 1 {
            return true;
        }
        //        println!("visit={}", word[0] as char);
        let row_count = board.len();
        let col_count = board[0].len();
        let next = Self::get_next((i, j), row_count, col_count);
        visit[i][j] = true;
        let res = next
            .iter()
            .any(|n| Self::dfs(board, &word[1..word.len()], n.0, n.1, visit));
        visit[i][j] = false;
        //        println!("visit={},res={}", word[0] as char, res);
        res
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
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let t = Solution::exist(v.clone(), "ABCCED".into());
        assert_eq!(t, true);
        let t = Solution::exist(v.clone(), "SEE".into());
        assert_eq!(t, true);
        let t = Solution::exist(v.clone(), "ABCB".into());
        assert_eq!(t, false);
        let v = vec![vec!['A']];
        let t = Solution::exist(v.clone(), "A".into());
        assert_eq!(t, true);
    }
}
