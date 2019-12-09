/*
39. 组合总和
给定一个无重复元素的数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。

candidates 中的数字可以无限制重复被选取。

说明：

所有数字（包括 target）都是正整数。
解集不能包含重复的组合。
示例 1:

输入: candidates = [2,3,6,7], target = 7,
所求解集为:
[
  [7],
  [2,2,3]
]
示例 2:

输入: candidates = [2,3,5], target = 8,
所求解集为:
[
[2,2,2,2],
[2,3,3],
[3,5]
]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/combination-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
[2,3,6,7] target=7为例
不断降低规模
7可以看作
2+5,3+4,6+1,7+0
然后分别对
1. 2,5 向下拆分
2. 3,4
3. 6,1
4. 7,0

为了为避免重复,可以使用hashMap来保存中间结果,比如6的拆分结果,
**这样会出现重复结果**
只要我们要求拆分结果是递增的就很容易进行裁剪,避免重复计算
[2,2,3]允许的,[3,2,3],[2,3,2]都是不允许的
*/

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        Self::combination(&candidates, 0, target)
    }
    fn combination(candidates: &Vec<i32>, select: i32, target: i32) -> Vec<Vec<i32>> {
        if candidates[0] > target {
            return Vec::new();
        }
        let mut res = Vec::new();
        for c in candidates {
            if *c < select {
                continue; //比如路径[2,2,3]只能是递增的,否则会出现重复
            }
            if *c == target {
                res.push(vec![target]);
                continue;
            }
            let left = Self::combination(candidates, *c, target - *c);
            if left.len() == 0 {
                continue;
            }
            //            println!("c={},left={:?}", c, left);
            //比如5 找到了各种组合
            for l in left {
                let mut r = Vec::new();
                r.push(*c);
                r.extend(l.iter());
                res.push(r)
            }
        }

        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        println!("t={:?}", t);
        let t = Solution::combination_sum(vec![2, 3, 5], 8);
        println!("t={:?}", t);
    }
}
