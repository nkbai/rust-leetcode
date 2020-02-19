/*
72 编辑距离
给定两个单词 word1 和 word2，计算出将 word1 转换成 word2 所使用的最少操作数 。

你可以对一个单词进行如下三种操作：

插入一个字符
删除一个字符
替换一个字符
示例 1:

输入: word1 = "horse", word2 = "ros"
输出: 3
解释:
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')
示例 2:

输入: word1 = "intention", word2 = "execution"
输出: 5
解释:
intention -> inention (删除 't')
inention -> enention (将 'i' 替换为 'e')
enention -> exention (将 'n' 替换为 'x')
exention -> exection (将 'n' 替换为 'c')
exection -> execution (插入 'u')

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/edit-distance
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
先暴力穷举
从后往前
s1: i=4
s2: j=3
如果s1[i]==s2[j],那么后退一步
如果不等,可以尝试三种情况.
1. 插入x
2. 删除
3. 替换
什么时候结束
i=0 并且j=0的情况下
*/

struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let s1 = word1.as_bytes();
        let s2 = word2.as_bytes();
        let i = s1.len() - 1;
        let j = s2.len() - 1;
        let mut m = vec![vec![-1; j + 1]; i + 1];
        return Self::dp(i as i32, j as i32, s1, s2, &mut m);
    }
    fn dp(i: i32, j: i32, s1: &[u8], s2: &[u8], m: &mut Vec<Vec<i32>>) -> i32 {
        if i < 0 {
            return j + 1; //s1已经走完了, 那么s2就还有剩余,直接插入
        }
        if j < 0 {
            return i + 1; //s2走完了,s1还有剩余,最块的肯定是全部删掉
        }
        if m[i as usize][j as usize] >= 0 {
            return m[i as usize][j as usize];
        }
        println!("try i={},j={}", i, j);
        if s1[i as usize] == s2[j as usize] {
            m[i as usize][j as usize] = Self::dp(i - 1, j - 1, s1, s2, m);
        } else {
            //从插入,删除,替换选最小的方案
            let l1 = Self::dp(i, j - 1, s1, s2, m); //插入
            let l2 = Self::dp(i - 1, j, s1, s2, m); //删除
            let l3 = Self::dp(i - 1, j - 1, s1, s2, m); //替换
            let mut l = l1;
            if l2 < l {
                l = l2;
            }
            if l3 < l {
                l = l3;
            }
            m[i as usize][j as usize] = l + 1;
        }
        return m[i as usize][j as usize];
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::min_distance("horse".into(), "ros".into());
        assert_eq!(t, 3);
        let t = Solution::min_distance("intention".into(), "execution".into());
        assert_eq!(t, 5);
    }
}
