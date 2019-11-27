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
如果sum[j]-sum[i]==k ,其中j>i 说明从i到j之间是一个等于k的子序列.
基于此思路可以一遍得到结果.
假设sum[j]=7,那么只需要找到5出现的次数,就是等于k的子序列的个数了.

这里有一个要求必须是j>i的情况下,因此必须是遍历一遍,一遍遍历,一遍计算.
如果先遍历统计sum,得到map,然后在处理,就会丢失顺序信息
*/

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut s = 0;
        let mut cnt = 0;
        let mut m = HashMap::new();

        *m.entry(0).or_insert(0) += 1;
        nums.iter().for_each(|n| {
            s += *n;
            if let Some(x) = m.get(&(s - k)) {
                cnt += *x;
            }
            *m.entry(s).or_insert(0) += 1;
        });
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
        let t = Solution::subarray_sum(vec![1], 0);
        assert_eq!(t, 0);
        let t = Solution::subarray_sum(vec![-1, -1, 1], 0);
        assert_eq!(t, 1);
        let t = Solution::subarray_sum(vec![-1, -1, 1], 1);
        assert_eq!(t, 1);
        let t = Solution::subarray_sum(vec![1, 2, 3], 3);
        assert_eq!(t, 2);
    }
}
