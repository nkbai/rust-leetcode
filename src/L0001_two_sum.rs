use std::collections::HashMap;

// mod L0009_palindrome_number; 
struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = HashMap::new();
        let mut answer = Vec::new();
        for (idx, num) in nums.iter().enumerate() {
            if let Some(&first) = cache.get(num) {
                answer.push(first);
                answer.push(idx as i32);
            } else {
                cache.insert(target - num, idx as i32);
            }
        }
        answer
    }
}

/*
给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。

示例:

给定 nums = [2, 7, 11, 15], target = 9

因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let nums = vec![2, 7, 11, 15];
        let target: i32 = 9;
        assert_eq!(vec![0, 1], Solution::two_sum(nums, target));
    }
}