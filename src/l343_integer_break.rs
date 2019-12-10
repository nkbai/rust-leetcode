/*
343. 整数拆分

给定一个正整数 n，将其拆分为至少两个正整数的和，并使这些整数的乘积最大化。 返回你可以获得的最大乘积。

示例 1:

输入: 2
输出: 1
解释: 2 = 1 + 1, 1 × 1 = 1。
示例 2:

输入: 10
输出: 36
解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。
说明: 你可以假设 n 不小于 2 且不大于 58。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/integer-break
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。*/

/*
思路:
1.
基本上是暴力递归回溯.
但是要加上记忆
2. 动态规划
避免了递归,效率上可能会高一点
3. 题解中有O(1)的解法,但是自己是想不出来的,就放弃吧.
*/

use std::cmp::max;

struct Solution {}
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; (n + 1) as usize];
        if n <= 3 {
            return (n - 1) as i32;
        }
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 3;
        for i in 4..=n {
            for j in 1..i {
                dp[i] = max(dp[i], dp[i - j] * dp[j])
            }
        }
        dp[n]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::integer_break(10), 36);
    }
}
