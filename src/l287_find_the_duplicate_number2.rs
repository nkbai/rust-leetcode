#![allow(unused_imports, dead_code)]
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
注意要求:
1. 不能更改原数组,所以不能排序
2. 只能使用O(1)空间
3. 时间复杂度小于O(N^2)
最暴力的方式:
针对a[i],在除a[i]之外的所有元素逐个比较. 这样肯定能找到重复的数字,但是违反了3,复杂度为O(N^2)
所以可以考虑二分查找:
第一遍在[1,n/2],(n/2,n]之间找,如果左边的个数超过n/2,则重复的数字一定在左边,否则在右边.
*/
struct Solution {}
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        //区间都是闭区间,左右都包含
        let mut l = (1, (nums.len() / 2) as i32);
        let mut r = ((nums.len() / 2 + 1) as i32, (nums.len() - 1) as i32);
        loop {
            let result = Self::find_internal(nums.as_slice(), l, r);
            println!("l={:?},r={:?},result={:?}", l, r, result);
            //在左边,加1是因为闭区间
            if result.0 > l.1 - l.0 + 1 {
                let len = l.1 - l.0;
                if len == 0 {
                    //长度只有1表示找到了
                    return l.0;
                }
                r = (len / 2 + l.0 + 1, l.1);
                l = (l.0, len / 2 + l.0);
            } else {
                //在右边
                let len = r.1 - r.0;
                if len == 0 {
                    return r.0; //长度只有1表示找到了
                }
                l = (r.0, len / 2 + r.0);
                r = (len / 2 + r.0 + 1, r.1);
            }
        }
        // panic!("not found");
    }
    fn find_internal(nums: &[i32], l: (i32, i32), r: (i32, i32)) -> (i32, i32) {
        let mut countl = 0;
        let mut countr = 0;
        nums.iter().for_each(|i| {
            if *i >= l.0 && *i <= l.1 {
                countl += 1;
            }
            if *i >= r.0 && *i <= r.1 {
                countr += 1;
            }
        });
        (countl, countr)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::find_duplicate(vec![2, 3, 4, 1, 2,]), 2);
        assert_eq!(Solution::find_duplicate(vec![2, 3, 1, 2,]), 2);
    }
}
