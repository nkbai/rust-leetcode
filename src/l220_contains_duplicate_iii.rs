/*
220. 存在重复元素 III
给定一个整数数组，判断数组中是否有两个不同的索引 i 和 j，使得 nums [i] 和 nums [j] 的差的绝对值最大为 t，并且 i 和 j 之间的差的绝对值最大为 ķ。

示例 1:

输入: nums = [1,2,3,1], k = 3, t = 0
输出: true
示例 2:

输入: nums = [1,0,1,1], k = 1, t = 2
输出: true
示例 3:

输入: nums = [1,5,9,1,5,9], k = 2, t = 3
输出: false

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/contains-duplicate-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
最简单粗暴的思路就是:
O(kN) 反复检索,
为了优化,新进来的数查找的时候,可以使用BTreeSet,使用二叉搜索树
*/
use std::cmp::min;
use std::collections::BTreeMap;
use std::ops::Bound::Included;
struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut s = BTreeMap::new();
        if t < 0 || k <= 0 || nums.len() <= 0 {
            return false;
        }
        let k = k as usize;
        s.insert(nums[0] as i64, 1);
        for i in 1..nums.len() {
            let n = nums[i] as i64;
            let min = n - t as i64;
            let max = n + t as i64;
            let r = s.range((Included(&min), Included(&max)));
            if r.count() > 0 {
                return true;
            }
            *s.entry(n).or_insert(0) += 1;
            if i >= k {
                //移除第i-k个,保证map里面不会超过k个元素
                let oldest = s.get_mut(&(nums[i - k] as i64)).unwrap();
                if *oldest == 1 {
                    //考虑到有多个的这种情况,比如1出现了多次
                    s.remove(&(nums[i - k] as i64));
                } else {
                    *oldest -= 1;
                }
            }
        }
        false
    }
    pub fn contains_nearby_almost_duplicate2(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 || nums.is_empty() || k <= 0 {
            return false;
        }
        let mut min = i32::max_value();
        let mut map: HashMap<i64, i32> = HashMap::new();

        for i in &nums {
            if *i < min {
                min = *i;
            }
        }

        let diff: i64 = t as i64 + 1;

        for iter in 0..nums.len() {
            let i = nums[iter];
            let index: i64 = ((i as i64) - (min as i64)) / diff;
            if let Some(left) = map.get(&(index - 1)) {
                if (i as i64 - (*left as i64)).abs() <= (t as i64) {
                    return true;
                }
            }

            if let Some(right) = map.get(&(index + 1)) {
                if (i as i64 - (*right as i64)).abs() <= (t as i64) {
                    return true;
                }
            }

            if map.get(&index).is_some() {
                return true;
            }

            map.insert(index, i);

            if iter as i32 >= k {
                map.remove(&(((nums[(iter as i32 - k) as usize] as i64) - (min as i64)) / diff));
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![0, 2147483647], 1, 2147483647),
            true
        );
    }
}
