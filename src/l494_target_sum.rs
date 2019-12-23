/*
494. 目标和

给定一个非负整数数组，a1, a2, ..., an, 和一个目标数，S。现在你有两个符号 + 和 -。对于数组中的任意一个整数，你都可以从 + 或 -中选择一个符号添加在前面。

返回可以使最终数组和为目标数 S 的所有添加符号的方法数。

示例 1:

输入: nums: [1, 1, 1, 1, 1], S: 3
输出: 5
解释:

-1+1+1+1+1 = 3
+1-1+1+1+1 = 3
+1+1-1+1+1 = 3
+1+1+1-1+1 = 3
+1+1+1+1-1 = 3

一共有5种方法让最终目标和为3。
注意:

数组非空，且长度不会超过20。
初始的数组的和不会超过1000。
保证返回的最终结果能被32位整数存下。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/target-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路
假设长度0.。n的子数组的，所有可能的组合产生的和保存一个map
其中key是和，value是出现的次数。
那么0..n+1后，这个map的扩充就是，先用a[n+1]为正，更新一边这个map
然后使用-a[n+1]再次更新，如果没有碰到重复的key，则插入，碰到重复的，则是
原来的基础上加1
所以这是一个动态规划的问题，并且下一个状态只与上一个有关，与上上个无关。

找个例子1，2，3，5
第一步：
1， -1
第二部：
3，1，-1，-3
第三步：
6，4，2，0，0，-2，-4，-6
第四步：
11，9，7，5，5，3，1，-1，1，-1，-3，-5，-5，-7，-9，-11
如果s=5，次数是2
s=-5 也是2
s=1，2
s=11,1
根据这个例子可以发现规律：
组合的和一定是关于0对称的，这个其实很容易想明白
我们只需要记录正数即可。

*/

struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let max_sum = 1000;
        if s > max_sum as i32 {
            return 0;
        }
        let mut dp = vec![0; 2 * max_sum + 2];
        dp[nums[0] as usize + max_sum] = 1; //初始化第一个数，的两种情况就是+，负数情况不考虑，根据对称即可
        dp[max_sum - nums[0] as usize] = dp[max_sum - nums[0] as usize] + 1; //考虑第一个数就是0的情况

        //题目中明确说了，和不会超过1000
        //假设0.。i可以凑出来的和为13（1），25（2），100（2），300（1）
        //那么信赖的i+1,直接在上面计算即可
        for i in 1..nums.len() {
            let n = nums[i] as usize;
            let dp2 = dp.clone();
            //加的情况
            for j in (0..2 * max_sum).rev() {
                if dp[j] == 0 {
                    continue;
                }
                //题中明确说了和不会超过1000
                let t = n + j;
                dp[t] = dp[j];
                if n != 0 {
                    dp[j] = 0; //考虑到n为0这种特殊情况
                }
            }
            for j in (0..2 * max_sum) {
                if dp2[j] == 0 {
                    continue;
                }
                //考虑减的情况
                let t = j - n;
                dp[t] += dp2[j];
            }
            println!("n={}======================", n);
            dp.iter().enumerate().for_each(|item| {
                if *item.1 > 0 {
                    println!("{}=>{}", item.0, item.1);
                }
            });
        }

        dp[(s + max_sum as i32) as usize]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reverse_list() {
        let t = Solution::find_target_sum_ways(vec![1, 2, 3, 5], 5);
        assert_eq!(t, 2);
        let t = Solution::find_target_sum_ways(vec![1, 0], 1);
        assert_eq!(t, 2);
        let t = Solution::find_target_sum_ways(vec![1000], -1000);
        assert_eq!(t, 1);
        let t = Solution::find_target_sum_ways(vec![0, 0, 0, 0, 0, 0, 0, 0, 1], 1);
        assert_eq!(t, 256);
    }
}
