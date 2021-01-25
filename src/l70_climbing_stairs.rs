#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
70. 爬楼梯
假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

注意：给定 n 是一个正整数。

示例 1：

输入： 2
输出： 2
解释： 有两种方法可以爬到楼顶。
1.  1 阶 + 1 阶
2.  2 阶
示例 2：

输入： 3
输出： 3
解释： 有三种方法可以爬到楼顶。
1.  1 阶 + 1 阶 + 1 阶
2.  1 阶 + 2 阶
3.  2 阶 + 1 阶

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/climbing-stairs
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
典型的最优子问题:
到达n不外乎两种方法,先到达n-1,然后走一个台阶,或者先到达n-2,再走两个台阶.
dp[n]=dp[n-1]+dp[n-2]
就是一个斐波那契数列.
*/
struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        }
        let mut n1 = 1;
        let mut n2 = 2;
        let mut i = 2;
        while i < n {
            let t = n1 + n2;
            n1 = n2;
            n2 = t;
            i += 1;
        }
        n2
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::climb_stairs(3);
        assert_eq!(t, 3);
        let t = Solution::climb_stairs(4);
        assert_eq!(t, 5);
    }
}
