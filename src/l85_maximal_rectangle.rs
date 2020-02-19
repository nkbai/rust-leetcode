/*
85. 最大矩形
给定一个仅包含 0 和 1 的二维二进制矩阵，找出只包含 1 的最大矩形，并返回其面积。

示例:

输入:
[
  ["1","0","1","0","0"],
  ["1","0","1","1","1"],
  ["1","1","1","1","1"],
  ["1","0","0","1","0"]
]
输出: 6

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximal-rectangle
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
使用一个矩阵来保存 每一个1位置对应的向左,向上连续1的个数,
当来一个新的点的时候
x x x1
x x2 O
比如求o,那么o向左最多有min(x1.0,x2.0+1),向上最多有x1.1+1个1
因此很容易求出面积
*/

struct Solution {}
struct LeftAndTop {
    left: i32,
    top: i32,
}
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut dp = Vec::new();
        if matrix.is_empty() {
            return 0;
        }
        //初始化min,建立同样的mxn数组
        matrix.iter().for_each(|_| {
            let mut v = Vec::new();
            matrix[0].iter().for_each(|_| {
                v.push(LeftAndTop { left: 0, top: 0 }); //向左是0,向上是1
            });
            dp.push(v);
        });
        let r = dp.len();
        let c = dp[0].len();
        let mut max_area = 0;
        //第一列
        for i in 0..r {
            let mut last = 0;
            if i >= 1 {
                last = dp[i - 1][0].top;
            }
            if matrix[i][0] == '1' {
                dp[i][0] = LeftAndTop {
                    left: 1,
                    top: last + 1,
                };
                if dp[i][0].top > max_area {
                    max_area = dp[i][0].top;
                }
            }
        }
        //第一行
        for i in 1..c {
            let last = dp[0][i - 1].left;
            if matrix[0][i] == '1' {
                dp[0][i] = LeftAndTop {
                    left: last + 1,
                    top: 1,
                };
                if dp[0][i].left > max_area {
                    max_area = dp[0][i].left;
                }
            }
        }
        for i in 1..r {
            for j in 1..c {
                if matrix[i][j] == '1' {
                    let mut left = dp[i][j - 1].left + 1;
                    if dp[i - 1][j].left != 0 && dp[i - 1][j].left < left {
                        left = dp[i - 1][j].left;
                    };
                    let top = dp[i - 1][j].top + 1;
                    let area = top * left;
                    dp[i][j] = LeftAndTop { left, top };
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }
        max_area
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ]);
        assert_eq!(t, 6);
        let t = Solution::maximal_rectangle(vec![vec!['1', '1']]);
        assert_eq!(t, 2);
        let t = Solution::maximal_rectangle(vec![
            vec!['0', '1', '1', '0', '1'],
            vec!['1', '1', '0', '1', '0'],
            vec!['0', '1', '1', '1', '0'],
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '1', '1', '1'],
            vec!['0', '0', '0', '0', '0'],
        ]);
        assert_eq!(t, 9);
    }
}
