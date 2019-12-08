/*
128. 最长连续序列
给定一个未排序的整数数组，找出最长连续序列的长度。

要求算法的时间复杂度为 O(n)。

示例:

输入: [100, 4, 200, 1, 3, 2]
输出: 4
解释: 最长连续序列是 [1, 2, 3, 4]。它的长度为 4。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-consecutive-sequence
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
从前往后,用一个map存储碰到的每一个数
value是这个数前面连续的数字和后面连续的数字
我们更新的时候,这样我们更新的时候只用更新最左边和最右边的数, 这些个数都包含自己
Hashmap的查询是O(1),因此时间复杂度是O(N)
*/
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

struct Solution {}
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut max_seq = 0;
        for v in nums {
            let mut e = m.get_mut(&v);
            if e.is_some() {
                continue;
            }
            let left = m.get(&(v - 1));
            let left_exist = left.is_some();
            let left = *left.unwrap_or(&(0, 0));
            let right = m.get(&(v + 1));
            let right_exist = right.is_some();
            let right = *right.unwrap_or(&(0, 0));
            let seq = left.0 + right.1 + 1;
            if max_seq < seq {
                max_seq = seq;
            }
            m.insert(v, (left.0 + 1, right.1 + 1));
            if left_exist {
                //一旦连起来,最左边的数的右边的数的个数需要大幅增加
                let mut left_most = m.get_mut(&(v - left.0)).unwrap();
                left_most.1 += 1 + right.1 //我+右边的数的右边个数
            }
            if right_exist {
                let right_most = m.get_mut(&(v + right.1)).unwrap();
                right_most.0 += 1 + left.0; //我+左边的数的左边个数
            }
        }

        max_seq
    }
}
#[cfg(test)]
mod test {
    #[warn(unused_imports)]
    use super::*;
    use crate::share::build_tree;
    #[test]
    fn test_generate() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
        assert_eq!(1, Solution::longest_consecutive(vec![100]));
        assert_eq!(1, Solution::longest_consecutive(vec![100, 102]));
        assert_eq!(2, Solution::longest_consecutive(vec![100, 102, 103]));
        assert_eq!(4, Solution::longest_consecutive(vec![100, 101, 102, 103]));
    }
}
