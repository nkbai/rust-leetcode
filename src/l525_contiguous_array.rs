/*
525. 连续数组

给定一个二进制数组, 找到含有相同数量的 0 和 1 的最长连续子数组（的长度）。



示例 1:

输入: [0,1]
输出: 2
说明: [0, 1] 是具有相同数量0和1的最长连续子数组。
示例 2:

输入: [0,1,0]
输出: 2
说明: [0, 1] (或 [1, 0]) 是具有相同数量0和1的最长连续子数组。


注意: 给定的二进制数组的长度不会超过50000。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/contiguous-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路
比如111011101
将0看做-1,然后记录从起始到现在的累加和,然后对应一个下标
那么上面的例子对应的就是
1  2  3  2  3  4  5  4  5
0  1  2  3  4  5  6  7  8
只需记录一个数第一次出现的次数,
比如如果第一次出现2,后面出现了两次2,那么最长的距离就是最后一次2的下标减去第一次的下标
*/

use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        let mut sum = 0;
        let mut max = 0;
        m.insert(0, -1); //初始和就是0,下标为-1,这样就算是有多个0也不会记错
        for i in nums.iter().enumerate() {
            if *i.1 == 0 {
                sum += -1;
            } else {
                sum += 1;
            }
            match m.entry(sum) {
                Entry::Occupied(e) => {
                    if i.0 as i32 - *e.get() > max {
                        max = i.0 as i32 - *e.get();
                    }
                }
                Entry::Vacant(mut e) => {
                    e.insert(i.0 as i32);
                }
            };
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share;
    #[test]
    fn test_reverse_list() {
        let t = Solution::find_max_length(vec![1, 1, 1, 0, 1, 1, 1, 0, 1]);
        assert_eq!(t, 2);
    }
}
