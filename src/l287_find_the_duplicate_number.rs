/*
287. 寻找重复数
给定一个包含 n + 1 个整数的数组 nums，其数字都在 1 到 n 之间（包括 1 和 n），可知至少存在一个重复的整数。假设只有一个重复的整数，找出这个重复的数。

示例 1:

输入: [1,3,4,2,2]
输出: 2
示例 2:

输入: [3,1,3,4,2]
输出: 3
说明：

不能更改原数组（假设数组是只读的）。
只能使用额外的 O(1) 的空间。
时间复杂度小于 O(n2) 。
数组中只有一个重复的数字，但它可能不止重复出现一次。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-the-duplicate-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

/*
思路
快慢指针法
把数组中的整数当做下标来处理就ok了.
比如
数值 1 3  4  2  2
下标 0 1  2  3  4
一开始pfast,pslow都是指向0,也就是1,然后不断往下走,如果碰到相同
说明存在循环.
*/
struct Solution {}
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut tortoise = nums[0];
        let mut hare = nums[0];

        loop {
            tortoise = nums[tortoise as usize];
            hare = nums[nums[hare as usize] as usize];
            if tortoise == hare {
                break;
            }
        }
        let mut p1 = nums[0];
        let mut p2 = tortoise;
        while p1 != p2 {
            p1 = nums[p1 as usize];
            p2 = nums[p2 as usize];
        }
        p1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::find_duplicate(vec![2, 3, 4, 1, 2,]), 2);
    }
}
