/*
152. 乘积最大子序列
给定一个整数数组 nums ，找出一个序列中乘积最大的连续子序列（该序列至少包含一个数）。

示例 1:

输入: [2,3,-2,4]
输出: 6
解释: 子数组 [2,3] 有最大乘积 6。
示例 2:

输入: [-2,0,-1]
输出: 0
解释: 结果不能为 2, 因为 [-2,-1] 不是子数组。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-product-subarray
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路一:
和累加和是一个思路,
1. 先求累加乘积,但是注意碰到0就将数组进行拆分,认为是不同的数组即可.
2. 从头开始,
因为要回去尝试每一种可能性,所以复杂度是O(N^2)
思路2:
时间复杂度O(N)
例如 -2 2 3 -2 4 -7
如果用dp[i] 表示前i个序列中以i结尾的最大子序列值,
那么dp[i+1]=max(dp[i]*nums[i+1],nums[i+1])
当然如果nums[i+1]是正数, 如果nums[i+1]是0,那么dp[i+1]就是0了
当然要考虑负数nums[i+1]有可能是负数,那这回导致最小的变成最大的.
因此d[i]要存两个值,一个是最小值,一个是最大值
因此dpmin[i+1]=min(dpmin[i]*nums[i+1],nums[i+1])
以刚刚的例子为例,最大值肯定是2,3,-2,4,-7,不能包含第一个-2
看一下过程
-2   2    3    -2     4      -7
-2   2    6    24    96      7*48
-2   -2   -12  -12   -48    -7*96

因为负负得正,所以只会碰到一次要不要包含的情形了,其他肯定是尽可能的都包含进去.
因此时间复杂度是O(N),空间复杂度是O(1)
至于0的情况,也很自然就解决了.
*/
use std::cmp::{max, min};
use std::mem::swap;

struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_value = 1;
        let mut min_value = 1;
        let mut m = std::i32::MIN;
        for i in 0..nums.len() {
            if nums[i] < 0 {
                swap(&mut max_value, &mut min_value);
            }
            max_value = max(max_value * nums[i], nums[i]);
            if max_value > m {
                m = max_value;
            }
            min_value = min(min_value * nums[i], nums[i]);
        }
        m
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
}
