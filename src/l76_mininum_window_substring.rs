/*
76. 最小覆盖子串
给你一个字符串 S、一个字符串 T，请在字符串 S 里面找出：包含 T 所有字母的最小子串。

示例：

输入: S = "ADOBECODEBANC", T = "ABC"
输出: "BANC"
说明：

如果 S 中不存这样的子串，则返回空字符串 ""。
如果 S 中存在这样的子串，我们保证它是唯一的答案。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-window-substring
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
此题和L30,L438都是类似的
看了题解关于滑动窗口的讲解,这次采用规范的滑动窗口来实现.
1. 一开始left,right=0
2. right向前走,直到[left:right]覆盖了整个字串
3. 然后left向前走,直到刚刚覆盖T,记下来作为结果的候选项.
复杂度是O(S+T)
1. 如何判断是否覆盖了整个字串
 建立一个map 保存需要的字符计数,每满足一个,计数加1,直到全部满足了.
2. 如何判断刚刚覆盖T
left向前走的时候,如果某个字符向下减1以后小于希望的数值,那么就是这个位置了.
*/

use std::collections::HashMap;
struct Term {
    expect: i32, //在匹配的过程中某个字符出现的次数
    count: i32,  // p中某个字符出现的次数
}
#[derive(Eq, PartialEq)]
enum ExpectResult {
    Less,
    Equal,
    Greater,
}
//管理map
impl Term {
    fn new() -> Self {
        Term {
            expect: 0,
            count: 0,
        }
    }
    fn inc_expect(&mut self) {
        self.expect += 1;
    }
    fn check_result(&self) -> ExpectResult {
        if self.expect > self.count {
            return ExpectResult::Less;
        } else if self.expect == self.count {
            return ExpectResult::Equal;
        } else {
            return ExpectResult::Greater;
        }
    }
    fn inc_count(&mut self) -> ExpectResult {
        self.count += 1;
        self.check_result()
    }
    fn dec_count(&mut self) -> ExpectResult {
        self.count -= 1;
        self.check_result()
    }
    #[allow(dead_code)]
    fn reset(&mut self) {
        self.count = 0;
    }
    #[allow(dead_code)]
    fn is_match(&self) -> bool {
        return self.expect == self.count;
    }
}

struct Solution {}
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut m = HashMap::new();
        for b in t {
            m.entry(b).or_insert(Term::new()).inc_expect();
        }
        let mut left = 0;
        let mut right = 0;
        let mut match_count = 0; //m中每一项匹配的次数
        let mut result = (std::usize::MIN, std::usize::MAX); //start,end

        while right < s.len() {
            m.entry(&s[right]).and_modify(|n| {
                if n.inc_count() == ExpectResult::Equal {
                    match_count += 1;
                }
            });
            if match_count == m.len() {
                //全部匹配上了,第2步满足要求,开始第三步
                while left <= right {
                    //left一直向前走,直到刚好不能覆盖T
                    m.entry(&s[left]).and_modify(|n| {
                        if n.dec_count() == ExpectResult::Less {
                            match_count -= 1;
                        }
                    });
                    if match_count < m.len() {
                        if right - left + 1 < result.1 - result.0 {
                            result = (left, right + 1);
                        }
                        left += 1;
                        break;
                    }
                    left += 1;
                }
            }
            right += 1;
        }

        if result.1 == std::usize::MAX {
            return String::new();
        } else {
            let ss = &s[result.0..result.1];
            return unsafe { String::from_utf8_unchecked(Vec::from(ss)) };
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        //        输入: S = "ADOBECODEBANC", T = "ABC"
        //        输出: "BANC"
        let t = Solution::min_window(String::from("ADOBECODEBANCDABCD"), String::from("ABC"));
        assert_eq!(t, String::from("ABC"));
        let t = Solution::min_window(String::from("a"), String::from("a"));
        assert_eq!(t, String::from("a"));
    }
}
