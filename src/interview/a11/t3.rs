/*
给定一个数组 nums ，如果 i < j 且 nums[i] > 2*nums[j] 我们就将 (i, j) 称作一个重要翻转对。

你需要返回给定数组中的重要翻转对的数量。

示例 1:

输入: [1,3,2,3,1]
输出: 2
示例 2:

输入: [2,4,3,5,1]
输出: 3
注意:

给定数组的长度不会超过50000。
输入数组中的所有数字都在32位整数的表示范围内。
*/
/*
思路:
1.从后往前
2.边走边排序,最好使用堆排序,还要能访问数组内部的值的那种
3. 碰到一个新来的,假设值=v,找小于v/2值的个数
4. 求和即可
*/
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_pairs(_nums: Vec<i32>) -> i32 {
        0
    }
}
#[cfg(test)]
mod test {
    #![allow(unused_imports, dead_code)]
    use super::*;

    #[test]
    fn test() {}
}
