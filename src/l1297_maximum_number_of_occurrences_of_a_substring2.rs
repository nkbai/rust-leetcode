use std::collections::HashMap;

#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
1297. 子串的最大出现次数

给你一个字符串 s ，请你返回满足以下条件且出现次数最大的 任意 子串的出现次数：

子串中不同字母的数目必须小于等于 maxLetters 。
子串的长度必须大于等于 minSize 且小于等于 maxSize 。


示例 1：

输入：s = "aababcaab", maxLetters = 2, minSize = 3, maxSize = 4
输出：2
解释：子串 "aab" 在原字符串中出现了 2 次。
它满足所有的要求：2 个不同的字母，长度为 3 （在 minSize 和 maxSize 范围内）。
示例 2：

输入：s = "aaaa", maxLetters = 1, minSize = 3, maxSize = 3
输出：2
解释：子串 "aaa" 在原字符串中出现了 2 次，且它们有重叠部分。
示例 3：

输入：s = "aabcabcab", maxLetters = 2, minSize = 2, maxSize = 3
输出：3
示例 4：

输入：s = "abcde", maxLetters = 2, minSize = 3, maxSize = 3
输出：0


提示：

1 <= s.length <= 10^5
1 <= maxLetters <= 26
1 <= minSize <= maxSize <= min(26, s.length)
s 只包含小写英文字母。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-number-of-occurrences-of-a-substring
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

 */

/*
思路:
逐个字符查找满足条件的字符串
因为规定了MaxSize为26,所以复杂度最大为O(N) =26N, N为字符串的长度
然后用一个map来统计每一个满足条件的字符串出现的次数.
空间复杂度也是O(N),只需存储每一个子串的指针即可

优化空间:
1. 很容易推导出,假设min_size=3,max_size=7
如果一个长度为5的子串出现次数是4,那么长度为3的子串出现次数一定是大于等于4
所以,可以忽略max_size这个参数
2. 严格每步都向前走一步,不回头
*/
struct Solution {}

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        if s.len() < min_size as usize {
            return 0;
        }
        let s = s.as_bytes();
        let mut m: HashMap<&[u8], usize> = HashMap::new();
        let max_letters = max_letters as usize;
        let min_size = min_size as usize;
        let _max_size = max_size as usize;
        let mut max = 0;
        let mut count = [0; 26];
        let mut cmax_letters = 0;
        for i in 0..min_size {
            let index = (s[i] - 'a' as u8) as usize;
            count[index] += 1;
            if count[index] == 1 {
                cmax_letters += 1;
            }
        }
        //用前min_size个字符来初始化
        if cmax_letters <= max_letters {
            m.insert(&s[0..min_size], 1);
            max = 1;
        }
        //满足长度要求
        for i in min_size..s.len() {
            let start = i - min_size + 1;
            let end = i;
            let pos_before_start = start - 1;
            let index = (s[i] - 'a' as u8) as usize;
            let pos_before_start_index = (s[pos_before_start] - 'a' as u8) as usize;
            count[pos_before_start_index] -= 1;
            if count[pos_before_start_index] == 0 {
                cmax_letters -= 1;
            }
            count[index] += 1;
            if count[index] == 1 {
                cmax_letters += 1;
            }
            //不满足字符个数的,跳过,比如max_letters=1,当前字符串是aba
            if cmax_letters > max_letters {
                continue;
            }
            let entry = m.entry(&s[start..=end]).or_default();
            *entry += 1;
            if max < *entry {
                max = *entry;
            }
            // println!(
            //     "found start={},end={},s={},count={}",
            //     start,
            //     end,
            //     unsafe { std::str::from_utf8_unchecked(&s[start..=end]) },
            //     *entry
            // );
        }
        max as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::max_freq("aababcaabz".into(), 2, 3, 4);
        assert_eq!(t, 2);
        let t = Solution::max_freq("aaaa".into(), 1, 3, 3);
        assert_eq!(t, 2);
        let t = Solution::max_freq("aabcabcab".into(), 2, 2, 3);
        assert_eq!(t, 3);
        let t = Solution::max_freq("abcde".into(), 2, 3, 3);
        assert_eq!(t, 0);
    }
}
