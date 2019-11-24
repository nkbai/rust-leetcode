/*
409. 最长回文串
给定一个包含大写字母和小写字母的字符串，找到通过这些字母构造成的最长的回文串。

在构造过程中，请注意区分大小写。比如 "Aa" 不能当做一个回文字符串。

注意:
假设字符串的长度不会超过 1010。

示例 1:

输入:
"abccccdd"

输出:
7

解释:
我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-palindrome
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
统计每个字符的个数
选一个单的,剩下的只能挑双的了.

简单起见,就用map,也可以用数组,只是注意要把大小写相应转换到对应的数组中去.
*/

use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let s = s.as_bytes();
        let mut m = HashMap::new();
        for i in s {
            *m.entry(i).or_insert(0) += 1;
        }
        let mut single = true;
        let mut cnt = 0;
        for (_k, v) in m {
            println!("k={},v={}", _k, v);
            if v % 2 == 1 && single {
                single = false;
                cnt += 1;
            }
            cnt += (v / 2) * 2; //只考虑偶数部分
        }
        cnt
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::longest_palindrome(String::from("abcccccdd")), 7);
        assert_eq!(Solution::longest_palindrome(String::from("ccc")), 3);
    }
}
