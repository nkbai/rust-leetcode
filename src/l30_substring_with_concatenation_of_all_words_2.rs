/*
30. 串联所有单词的子串

给定一个字符串 s 和一些长度相同的单词 words。找出 s 中恰好可以由 words 中所有单词串联形成的子串的起始位置。

注意子串要与 words 中的单词完全匹配，中间不能有其他字符，但不需要考虑 words 中单词串联的顺序。



示例 1：

输入：
  s = "barfoothefoobarman",
  words = ["foo","bar"]
输出：[0,9]
解释：
从索引 0 和 9 开始的子串分别是 "barfoo" 和 "foobar" 。
输出的顺序不重要, [9,0] 也是有效答案。
示例 2：

输入：
  s = "wordgoodgoodgoodbestword",
  words = ["word","good","best","word"]
输出：[]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/substring-with-concatenation-of-all-words
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
1. 题目明确要求的是words中的字符串长度都是固定的,且相同
2. words中的字符串会重复
3. 使用map来对words中的每个字符串进行计数统计
4. 以单词为单位对s中的字符串进行分割,只要以某个字符串开始,能够让3中的map计数器降到0,那么就认为找到了一个位置

被题解误导
*/

use std::collections::hash_map::Entry;
use std::collections::HashMap;
struct Term {
    expect: i32,
    count: i32,
}
impl Term {
    fn new(expect: i32, count: i32) -> Self {
        Term { expect, count }
    }
    fn inc_expect(&mut self) {
        self.expect += 1;
    }
    fn inc(&mut self) {
        self.count += 1;
    }
    fn dec(&mut self) {
        self.count -= 1;
    }
    fn exhausted(&self) -> bool {
        self.count > self.expect
    }
    fn reset(&mut self) {
        self.count = 0;
    }
}
struct Solution {}
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.len() < 1 {
            return vec![];
        }
        let word_len = words[0].len();
        if word_len < 1 {
            return vec![];
        }
        let mut map: HashMap<&str, Term> = HashMap::with_capacity(words.len());
        for word in words.iter() {
            map.entry(word).or_insert(Term::new(0, 0)).inc_expect();
        }
        let mut ret: Vec<i32> = Vec::new();
        for shift in 0..word_len {
            let mut i = shift;
            let mut j = shift;
            while j + word_len - 1 < s.len() {
                match map.entry(&s[j..j + word_len]) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().inc();
                        if entry.get().exhausted() {
                            while i < j {
                                let term = &s[i..i + word_len];
                                map.entry(term).and_modify(|t| t.dec());
                                i += word_len;
                                if term == &s[j..j + word_len] {
                                    break;
                                }
                            }
                            j += word_len;
                        } else {
                            if j - i < (words.len() - 1) * word_len {
                                j += word_len;
                            } else {
                                ret.push(i as i32);
                                map.entry(&s[i..i + word_len]).and_modify(|t| t.dec());
                                j += word_len;
                                i += word_len;
                            }
                        }
                    }
                    Entry::Vacant(_entry) => {
                        map.iter_mut().for_each(|(_, v)| v.reset());
                        j += word_len;
                        i = j;
                    }
                }
            }
            map.iter_mut().for_each(|(_, v)| v.reset())
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::find_substring(
                String::from("barfoothefoobarman"),
                vec![String::from("foo"), String::from("bar")]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("word")
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("good")
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                String::from("ababaab"),
                vec![String::from("ab"), String::from("ba"), String::from("ba")]
            ),
            vec![1]
        );
    }
}
