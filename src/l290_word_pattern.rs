/*
290. 单词规律

给定一种规律 pattern 和一个字符串 str ，判断 str 是否遵循相同的规律。

这里的 遵循 指完全匹配，例如， pattern 里的每个字母和字符串 str 中的每个非空单词之间存在着双向连接的对应规律。

示例1:

输入: pattern = "abba", str = "dog cat cat dog"
输出: true
示例 2:

输入:pattern = "abba", str = "dog cat cat fish"
输出: false
示例 3:

输入: pattern = "aaaa", str = "dog cat cat dog"
输出: false
示例 4:

输入: pattern = "abba", str = "dog dog dog dog"
输出: false
说明:
你可以假设 pattern 只包含小写字母， str 包含了由单个空格分隔的小写字母。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/word-pattern
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
很简单,严格按照pattern来就行了,
在str先用空格分隔
建立map[字符串]=>pattern中字母
然后每一个字符串如果不能严格匹配就结束.

考虑到题目中提示只能是小写字母,用数组不用map也是ok的
*/

//一开始没有考虑到pattern中不同的key对应相同的字符串问题
use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern: Vec<char> = pattern.chars().collect();
        let s: Vec<&str> = s.split(' ').collect();
        if pattern.len() != s.len() || pattern.len() == 0 {
            return false;
        }
        let mut arr: [&str; 26] = [""; 26];

        for i in 0..pattern.len() {
            let p = pattern[i] as usize - 'a' as usize;
            if arr[p].is_empty() {
                arr[p] = s[i];
            } else if arr[p] != s[i] {
                return false;
            }
        }
        let mut m = HashSet::new();
        for i in 0..arr.len() {
            if m.contains(arr[i]) && arr[i].len() != 0 {
                return false;
            }
            m.insert(arr[i]);
        }
        true
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat fish")),
            false
        );
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat dog")),
            true
        );
    }
}
