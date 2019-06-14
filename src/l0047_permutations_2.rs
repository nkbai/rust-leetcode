/*
全排列 II https://leetcode-cn.com/problems/permutations-ii/

给定一个可包含重复数字的序列，返回所有不重复的全排列。

示例:

输入: [1,1,2]
输出:
[
  [1,1,2],
  [1,2,1],
  [2,1,1]
]



*/

/*

思路:

因为要穷举结果,所以只能用穷举的办法.
思路:
对于完全不重复的数字,找到所有可能组合的办法:
[1]
对于1,2
则是
[2,1]
[1,2]
于1,2,3
则是
[3,2,1] [2,3,1],[2,1,3] 遍历前一个,在所有可能的位置插入新来的数字即可.
[3,1,2] [1,3,2] [1,2,3]

这种方式的坏处是要不停地移动,插入,

从头开始的方法:
首先选定最开始的位置,还以[1,2,3]举例,那么
一开始是:
1 [2,3]组合
2 [1,3]组合
3 [1,2]组合
如果碰到重复的数字直接跳过进行下一个.
这样一轮下来以后[1,2,3]变成了[3,1,2],需要修复一下.
*/
struct Solution {}
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort();
        let nl = nums.len();
        return Solution::permute_interna(&mut nums[0..nl], nl);
    }
    fn permute_interna(nums: &mut [i32], n: usize) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            let mut v = vec![0; n];
            v[n - 1] = nums[0];
            let mut vs = Vec::new();
            vs.push(v);
            return vs;
        }
        let mut vs2 = Vec::new();
        let nl = nums.len();
        //        println!("enter={:?}", nums);
        //        let mut nums2 = Vec::new();
        //        for i in 0..nl {
        //            nums2.push(nums[i]);
        //        }
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[0] {
                continue; //重复的数字
            }
            let t = nums[i];
            nums[i] = nums[0];
            nums[0] = t;
            let mut vs = Solution::permute_interna(&mut nums[1..nl], n);
            for v in &mut vs {
                v[n - nums.len()] = nums[0]; //把刚刚选定的队首放进去
            }
            vs2.append(&mut vs)
        }
        //nums里面的顺序回复一下
        let t = nums[0];
        for i in 1..nl {
            nums[i - 1] = nums[i]; //有没有现成的函数可以使用呢?
        }
        nums[nl - 1] = t;
        //        println!("exit={:?}", nums);
        //        assert_eq!(nums, &mut nums2[0..nl]);
        vs2
    }
}

mod test {
    use super::*;
    #[test]
    fn test_permute() {
        println!("v={:?}", Solution::permute_unique(vec![2, 2, 1, 1]))
    }
}
