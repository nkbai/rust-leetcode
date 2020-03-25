use std::collections::HashMap;

#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
219. 存在重复元素 II

给定一个整数数组和一个整数 k，判断数组中是否存在两个不同的索引 i 和 j，使得 nums [i] = nums [j]，并且 i 和 j 的差的 绝对值 至多为 k。



示例 1:

输入: nums = [1,2,3,1], k = 3
输出: true
示例 2:

输入: nums = [1,0,1,1], k = 1
输出: true
示例 3:

输入: nums = [1,2,3,1,2,3], k = 2
输出: false

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/contains-duplicate-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
从前往后扫描,只记录一个数最后一次出现的下标
如果再次出现,那么下标相减,满足k则返回true
否则返回false
*/
struct Solution {}
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut m = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match m.get(v) {
                None => {
                    m.insert(*v, i);
                }
                Some(j) => {
                    if i - *j <= k as usize {
                        return true;
                    }
                    m.insert(*v, i);
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3);
        assert_eq!(t, true);
    }
}
