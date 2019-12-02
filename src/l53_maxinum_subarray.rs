/*
53. 最大子序和

给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

示例:

输入: [-2,1,-3,4,-1,2,1,-5,4],
输出: 6
解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。
进阶:

如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的分治法求解。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-subarray
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
首先保存一个当前最大和
从前往后走的时候,碰到一个新的数值假定a[i]
那么计算sum=max(a[i],a[i]+sum),作为继续往下推进的条件.
*/
use std::cmp::max;

struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = std::i64::MIN;
        let mut sum = std::i32::MIN as i64;
        for n in nums {
            sum = max(sum + n as i64, n as i64);
            //            println!("n={},sum={}", n, sum);
            if sum > max_sum {
                max_sum = sum;
            }
        }
        max_sum as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let t = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(t, 6);
    }
}
