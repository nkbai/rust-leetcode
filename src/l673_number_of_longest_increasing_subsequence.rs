/*
673. 最长递增子序列的个数
给定一个未排序的整数数组，找到最长递增子序列的个数。

示例 1:

输入: [1,3,5,4,7]
输出: 2
解释: 有两个最长递增子序列，分别是 [1, 3, 4, 7] 和[1, 3, 5, 7]。
示例 2:

输入: [2,2,2,2,2]
输出: 5
解释: 最长递增子序列的长度是1，并且存在5个子序列的长度为1，因此输出5。
注意: 给定的数组长度不超过 2000 并且结果一定是32位有符号整数。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/number-of-longest-increasing-subsequence
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
dp[i]表示以i结尾的递增子序列的最大长度以及以及个数
比如
2 1 3 4 5
dp[0]={1,1}
dp[1]={1,1}
dp[2]={2,2}
dp[4]={4,2}
那么dp[j]=
遍历0..j之间所有的子序列,如果a[i]<a[j],则取其最大值+1,同时累加相关个数
然后从头遍历,统计最长递增子序列对应的个数
*/
struct Solution {}
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut max_len = 1;
        let mut dp = Vec::new();
        dp.push((1, 1));
        for j in 1..nums.len() {
            let nj = nums[j];
            let mut current_max = 1;
            let mut current_max_count = 1;
            for i in 0..j {
                let cmax = dp[i].0 + 1;
                if nums[i] < nj && cmax >= current_max {
                    if cmax == current_max {
                        current_max_count += dp[i].1; //相同的长度要累加
                    } else {
                        current_max = cmax;
                        current_max_count = dp[i].1;
                    }
                }
            }
            println!(
                "current={},current_max={},current_max_count={}",
                nj, current_max, current_max_count
            );
            dp.push((current_max, current_max_count));
            if max_len < current_max {
                max_len = current_max;
            }
        }
        let mut count = 0;
        for (max, c) in dp {
            if max == max_len {
                count += c;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
        assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
    }
}
