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

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut m = HashMap::new();
        if words.len() == 0 {
            return Vec::new(); //空的字符串,不用找
        }
        if words[0].len() == 0 {
            return Vec::new(); //字符串的长度是0,也不用找了
        }
        let sub_len = words[0].len();
        let mut total_len = 0;
        for s in words.iter() {
            *m.entry(s.as_bytes()).or_insert(0) += 1;
            total_len += sub_len;
        }
        let mut i = 0;
        let mut res = Vec::new();
        let s = s.as_bytes();
        while i + total_len <= s.len() {
            let mut m2 = m.clone();
            let mut j = i;
            while m2.len() > 0 && j + sub_len <= s.len() {
                println!("i={},j={}", i, j);
                //应该是rust版本问题,低版本必须先独立声明r,不能直接使用s[j..(j+sub_len)]
                let r = j..(j + sub_len);
                let sub = &s[r];
                if m2.contains_key(sub) {
                    let e = m2.get_mut(sub).expect("must have");
                    *e -= 1;
                    if *e == 0 {
                        m2.remove(sub);
                    }
                } else {
                    break;
                }
                j += sub_len;
            }
            //如果全部都匹配到了,这时候m2肯定空了.
            //如果匹配到了,可以考虑跳的更多,
            //比如words=[aa,bb,cc]
            //如果i匹配到了,比如s[i:i+sub_len]=cc,那么只需要看s[i+sub_len*2:i+sub_len*3]是否是cc
            //如果是cc,自然可以继续匹配,如果不是,还要分两种情况
            //1. s[i+sub_len*2:i+sub_len*3]是否在m中,如果不在,跳过
            //2.如果在,谨慎起见,只能继续从i+sub_len开始匹配,
            if m2.is_empty() {
                res.push(i as i32);
            }
            i += 1;
        }
        res
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
