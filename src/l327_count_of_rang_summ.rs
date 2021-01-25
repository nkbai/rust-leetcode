#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
327. 区间和的个数
给定一个整数数组 nums，返回区间和在 [lower, upper] 之间的个数，包含 lower 和 upper。
区间和 S(i, j) 表示在 nums 中，位置从 i 到 j 的元素之和，包含 i 和 j (i ≤ j)。

说明:
最直观的算法复杂度是 O(n2) ，请在此基础上优化你的算法。

示例:

输入: nums = [-2,5,-1], lower = -2, upper = 2,
输出: 3
解释: 3个区间分别是: [0,0], [2,2], [0,2]，它们表示的和分别为: -2, -1, 2。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/count-of-range-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
对前缀和进行排序,同时记住下标.
然后找寻区间的按顺序查找即可.

空间复杂度O(N)
时间复杂度O(N^2) 最糟糕的情形下是这个,最糟糕的情形是lower,upper非常大,
导致要穷举所有的区间,这个过程本身就是O(N^2)级别的复杂度.

所以说无论哪种算法,对于本题而言,最糟糕的情形复杂度都是O(N^2)
*/
struct Solution {}
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // let mut sum_vec = vec![];
        // let mut sum = 0;
        // nums.iter().enumerate().for_each(|(pos, v)| {
        //     sum += *v;
        //     sum_vec.push((sum, pos));
        // });
        // sum_vec.sort(); // 对于累加和从小到大进行排序.
        // let mut count = 0;
        // for i in 0..nums.len() {
        //     let mut upper_bound = sum_vec[j] + upper;
        //     let mut upper_pos = j;
        //     //如果upper>=0,往前找,全部都满足条件.
        //     if upper < 0 {
        //         //二分查找,找出最后一个大于upper_bound的.
        //     }
        //     let mut lower_bound = sum_vec[j] + lower;
        //     //二分查找,找出第一个小于lower的
        //
        //     for j in (0..i + 1).rev() {
        //         let ji = sum_vec[j] - sum_vec[i];
        //         //往后走,只会差距变得更大.
        //         if ji > upper {
        //             break;
        //         }
        //     }
        // }
        0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        println!("ok");
        for i in 0..1 {
            println!("i={}", i);
        }
    }
}
