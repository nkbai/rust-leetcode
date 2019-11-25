/*
560. 和为K的子数组
给定一个整数数组和一个整数 k，你需要找到该数组中和为 k 的连续的子数组的个数。

示例 1 :

输入:nums = [1,1,1], k = 2
输出: 2 , [1,1] 与 [1,1] 为两种不同的情况。
说明 :

数组的长度为 [1, 20,000]。
数组中元素的范围是 [-1000, 1000] ，且整数 k 的范围是 [-1e7, 1e7]。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/subarray-sum-equals-k
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
比如
1,2,-1,1,-1,2
那么k=2的连续子数组
从1开始的有1,2,-1| 1,2,-1,1,-1
然后从2开始
有2 | 2,-1,1
....
针对第一个1,使用sum来保存从1到当前位置的和
得到sum=[1,3,2,3,2,4]
因此满足条件的有两个.
考虑第二个2,
那么sum应该是每个数依次减1,得到
[2,1,2,1,3]
以此类推
时间复杂度是O(N^2),空间复杂度是O(N)
*/

struct Solution {}
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = Vec::with_capacity(nums.len());
        let mut s = 0;
        let mut cnt = 0;
        nums.iter().for_each(|n| {
            s += *n;
            sum.push(s);
            if s == k {
                cnt += 1;
            }
        });
        if nums.len() == 0 {
            return cnt;
        }
        let mut last_dec = nums[0];
        for i in 1..nums.len() {
            for j in i..nums.len() {
                sum[j] -= nums[i - 1];
                if sum[j] == k {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::subarray_sum(vec![1, 1, 1], 2);
        assert_eq!(t, 2);
        let t = Solution::subarray_sum(vec![1, 2, -1, 1, -1, 2], 2);
        assert_eq!(t, 6);
    }
}
