/*
97. 交错字符串
给定三个字符串 s1, s2, s3, 验证 s3 是否是由 s1 和 s2 交错组成的。

示例 1:

输入: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
输出: true
示例 2:

输入: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
输出: false
*/
/*
思路:
动态规划
dp[i][j]=k>0 表示s1[0..i],s2[0..j]对应s3[0..k]
那么
dp[i+1][j]= k+1 if s1[i+1]==s3[k+1] else -1
dp[i][j+1]=k+1 if s2[j+1]==s3[k+1] else -1
*/

struct Solution {}
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        if s1.len() == 0 {
            return s2 == s3;
        }
        if s2.len() == 0 {
            return s1 == s3;
        }
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = 0;
        for i in 0..s1.len() + 1 {
            for j in 0..s2.len() + 1 {
                /*
                dp[i+1][j]= k+1 if s1[i+1]==s3[k+1] else -1
                dp[i][j+1]=k+1 if s2[j+1]==s3[k+1] else -1
                */
                if i > 0 && dp[i - 1][j] >= 0 {
                    let k = dp[i - 1][j] as usize;
                    if s1[i - 1] == s3[k] {
                        dp[i][j] = (k + 1) as i32;
                    }
                }
                if j > 0 && dp[i][j - 1] >= 0 {
                    let k = dp[i][j - 1] as usize;
                    if s2[j - 1] == s3[k] {
                        dp[i][j] = (k + 1) as i32;
                    }
                }
            }
        }
        println!("dp={:?}", dp);
        dp[s1.len()][s2.len()] == s3.len() as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        //        let t = Solution::is_interleave("aabcc".into(), "dbbca".into(), "aadbbcbcac".into());
        //        assert_eq!(t, true);
        //        let t = Solution::is_interleave("aabcc".into(), "dbbca".into(), "aadbbbaccc".into());
        //        assert_eq!(t, false);
        let t = Solution::is_interleave("a".into(), "b".into(), "a".into());
        assert_eq!(t, false);
    }
}
