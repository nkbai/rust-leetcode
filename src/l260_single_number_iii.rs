/*
260. 只出现一次的数字 III
给定一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。

示例 :

输入: [1,2,1,3,2,5]
输出: [3,5]
注意：

结果输出的顺序并不重要，对于上面的例子， [5, 3] 也是正确答案。
你的算法应该具有线性时间复杂度。你能否仅使用常数空间复杂度来实现？
在真实的面试

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/single-number-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
1,2,1,3,2,5
第一遍所有的数都异或,会得到s=3 xor 5
这时候x有一个非常好的特性s xor 3 =5; s xor 5=3
但是s xor 1 有可能是数组中的其他数,姑且称之为t,
*/

struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        panic!("想不明白,这种小技巧还是放弃吧");
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {}
}
