use std::cmp::max;

#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
1143. 最长公共子序列
给定两个字符串 text1 和 text2，返回这两个字符串的最长公共子序列。

一个字符串的 子序列 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。两个字符串的「公共子序列」是这两个字符串所共同拥有的子序列。

若这两个字符串没有公共子序列，则返回 0。



示例 1:

输入：text1 = "abcde", text2 = "ace"
输出：3
解释：最长公共子序列是 "ace"，它的长度为 3。
示例 2:

输入：text1 = "abc", text2 = "abc"
输出：3
解释：最长公共子序列是 "abc"，它的长度为 3。
示例 3:

输入：text1 = "abc", text2 = "def"
输出：0
解释：两个字符串没有公共子序列，返回 0。


提示:

1 <= text1.length <= 1000
1 <= text2.length <= 1000
输入的字符串只含有小写英文字符。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-common-subsequence
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
和编辑距离类似
首先具有最优子问题,bcde和ce的最长公共子序列是2,那么abcde和ace的最长公共子序列就是3
dp[i][j] 表示第一个字符串从i到结尾,第二个字符串从j到结尾的最长公共子序列.
那么最终答案是dp[0][0]
dp[i][j]=  1+dp[i+1][j+1] if  s1[i]==s2[j] else
           max(dp[i+1][j],dp[i][j+1] )

*/
struct Solution {}
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let s1 = text1.as_bytes();
        let s2 = text2.as_bytes();
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i in (0..s1.len()).rev() {
            for j in (0..s2.len()).rev() {
                if s1[i] == s2[j] {
                    let t1 = 1 + dp[i + 1][j + 1];
                    let t2 = max(dp[i + 1][j], dp[i][j + 1]);
                    dp[i][j] = max(t1, t2);
                } else {
                    dp[i][j] = max(dp[i + 1][j], dp[i][j + 1]);
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
        let t = Solution::longest_common_subsequence("abcde".into(), "ace".into());
        assert_eq!(t, 3);
        let t = Solution::longest_common_subsequence("abc".into(), "abc".into());
        assert_eq!(t, 3);
        let t = Solution::longest_common_subsequence("abc".into(), "def".into());
        assert_eq!(t, 0);
    }
}
