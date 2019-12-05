/*
523. 连续的子数组和
给定一个包含非负数的数组和一个目标整数 k，编写一个函数来判断该数组是否含有连续的子数组，其大小至少为 2，总和为 k 的倍数，即总和为 n*k，其中 n 也是一个整数。

示例 1:

输入: [23,2,4,6,7], k = 6
输出: True
解释: [2,4] 是一个大小为 2 的子数组，并且和为 6。
示例 2:

输入: [23,2,6,4,7], k = 6
输出: True
解释: [23,2,6,4,7]是大小为 5 的子数组，并且和为 42。
说明:

数组的长度不会超过10,000。
你可以认为所有数字总和在 32 位有符号整数范围内。
*/

/*
思路倒着往前
要求必须是连续的子数组
复杂度为O(N^2)
*/

struct Solution {}

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut v = Vec::new();

        if nums.len() <= 1 {
            return false;
        }
        v.push(nums[nums.len() - 1]);
        for i in (0..nums.len() - 1).rev() {
            for j in v.iter_mut() {
                *j += nums[i];
                if (k != 0 && *j % k == 0) || *j == 0 {
                    return true;
                }
            }
            v.push(nums[i]);
        }
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::check_subarray_sum(vec![23, 2, 7, 6, 7], 6);
        assert_eq!(t, false);
        let t = Solution::check_subarray_sum(vec![0, 0], 0);
        assert_eq!(t, true);
    }
}
