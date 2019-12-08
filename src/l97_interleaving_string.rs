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
暴力递归回溯,可以有不少优化空间
*/

struct Solution {}
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        return Self::is_interleav_internal(s1, s2, s3);
    }
    fn is_interleav_internal(s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
        //直到走到最后
        if s1.len() == 0 {
            return s2 == s3;
        }
        if s2.len() == 0 {
            return s1 == s3;
        }
        if s1[0] == s3[0] {
            if Self::is_interleav_internal(&s1[1..], s2, &s3[1..]) {
                return true;
            }
        }
        if s2[0] == s3[0] {
            if Self::is_interleav_internal(s1, &s2[1..], &s3[1..]) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let t = Solution::is_interleave("aabcc".into(), "dbbca".into(), "aadbbcbcac".into());
        assert_eq!(t, true);
        let t = Solution::is_interleave("aabcc".into(), "dbbca".into(), "aadbbbaccc".into());
        assert_eq!(t, false);
    }
}
