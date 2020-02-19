/*
给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？找出所有满足条件且不重复的三元组。

注意：答案中不可以包含重复的三元组。

例如, 给定数组 nums = [-1, 0, 1, 2, -1, -4]，

满足要求的三元组集合为：
[
  [-1, 0, 1],
  [-1, -1, 2]
]
*/
use std::collections::HashMap;
struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        //num->hash<index>
        let mut cache = HashMap::new();
        let mut answer: Vec<Vec<i32>> = Vec::new();
        nums.sort();
        for i in 0..nums.len() {
            //            if let Some(x) = cache.get(&nums[i]) {
            //                cache.insert(nums[i], );
            //            } else {
            //                cache.insert(nums[i], 1);
            //            }
            cache.insert(nums[i], i);
        }
        println!("nums={:?}", nums);
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let x = 0 - (nums[i] + nums[j]);
                if let Some(y) = cache.get(&x) {
                    //存在匹配,遍历,确认i,j下标不在里面
                    //其实只有三个0这种特殊情况会是i,j,k值都一样,否则不可能会出现相同
                    if *y > j {
                        answer.push(vec![nums[i], nums[j], x]);
                    }
                }
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let l1 = vec![-1, 0, 1, 2, -1, -4];
        let l2 = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(l2, Solution::three_sum(l1));
        let l1 = vec![0, 0, 0, 0, 0, 0];
        let l2 = vec![vec![0, 0, 0]];
        assert_eq!(l2, Solution::three_sum(l1));
    }
}
