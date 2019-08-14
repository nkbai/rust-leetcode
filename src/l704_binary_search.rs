/*
704. 二分查找
https://leetcode-cn.com/problems/binary-search/
给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。

```
示例 1:

输入: nums = [-1,0,3,5,9,12], target = 9
输出: 4
解释: 9 出现在 nums 中并且下标为 4
示例 2:

输入: nums = [-1,0,3,5,9,12], target = 2
输出: -1
解释: 2 不存在 nums 中因此返回 -1
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-search
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut low = 0usize;
        let mut high = nums.len() - 1;
        while low <= high {
            let mid = (low + high) / 2;
            if nums[mid] > target {
                if mid == 0 {
                    //0要特殊处理,否则会造成panic
                    return -1;
                }
                high = mid - 1;
            } else if nums[mid] < target {
                low = mid + 1;
            } else {
                return mid as i32;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search() {
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
        assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
        assert_eq!(-1, Solution::search(vec![-1, 1], 2));
        assert_eq!(-1, Solution::search(vec![2], -2));
    }
}
