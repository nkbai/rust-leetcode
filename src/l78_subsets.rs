/*
78. 子集

给定一个可能包含重复元素的整数数组 nums，返回该数组所有可能的子集（幂集）。

说明：解集不能包含重复的子集。

示例:

输入: [1,2,2]
输出:
[
  [2],
  [1],
  [1,2,2],
  [2,2],
  [1,2],
  []
]
*/
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut m = HashMap::new();
        for n in nums.as_slice() {
            *m.entry(*n).or_insert(0) += 1;
        }
        res.push(Vec::new()); //空集
        for (k, cnt) in m {
            let mut local = Vec::new();
            for i in 0..cnt {
                local.push(vec![k; i + 1]);
            }
            res = Self::extends(&res, &local);
            println!("local={:?},res={:?}", local, res);
        }
        res
    }
    fn extends(res: &Vec<Vec<i32>>, local: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut v = Vec::new();
        for r in res.as_slice() {
            v.push(r.clone());
            for l in local.as_slice() {
                let mut r = r.clone();
                r.extend(l);
                v.push(r);
            }
        }
        v
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::subsets_with_dup(vec![1, 1, 2, 2, 2]);
        println!("t={:?}", t);
    }
}
