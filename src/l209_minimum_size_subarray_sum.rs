/*
209. 长度最小的子数组
给定一个含有 n 个正整数的数组和一个正整数 s ，找出该数组中满足其和 ≥ s 的长度最小的连续子数组。如果不存在符合条件的连续子数组，返回 0。

示例:

输入: s = 7, nums = [2,3,1,2,4,3]
输出: 2
解释: 子数组 [4,3] 是该条件下的长度最小的连续子数组。
进阶:

如果你已经完成了O(n) 时间复杂度的解法, 请尝试 O(n log n) 时间复杂度的解法。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-size-subarray-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路
首先求累加和 O(N)
然后再从头开始找到第一个大于等于s的位置
记下来第一个最小子数组.
假设start指向0,end指向大于s的位置
最终start从头快走到尾,end是从头走到尾,所以复杂度是O(N)
*/
struct Solution {}
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        if nums.len() == 0 {
            return 0;
        }
        for i in 1..nums.len() {
            nums[i] = nums[i - 1] + nums[i];
        }
        if nums[nums.len() - 1] < s {
            return 0;
        }
        if nums[0] >= s {
            return 1;
        }
        let mut start: i32 = -1;
        let mut start_value = 0;
        let mut end = 0;
        let mut min_len = std::i32::MAX;
        while end < nums.len() {
            if nums[end] - start_value < s {
                end += 1;
                continue;
            }
            let l = end as i32 - start;
            if l < min_len {
                min_len = l;
            }
            start += 1;
            start_value = nums[start as usize];
        }
        min_len as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);
    }
}
