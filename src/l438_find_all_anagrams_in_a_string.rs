/*
438. 找到字符串中所有字母异位词

给定一个字符串 s 和一个非空字符串 p，找到 s 中所有是 p 的字母异位词的子串，返回这些子串的起始索引。

字符串只包含小写英文字母，并且字符串 s 和 p 的长度都不超过 20100。

说明：

字母异位词指字母相同，但排列不同的字符串。
不考虑答案输出的顺序。
示例 1:

输入:
s: "cbaebabacd" p: "abc"

输出:
[0, 6]

解释:
起始索引等于 0 的子串是 "cba", 它是 "abc" 的字母异位词。
起始索引等于 6 的子串是 "bac", 它是 "abc" 的字母异位词。
示例 2:

输入:
s: "abab" p: "ab"

输出:
[0, 1, 2]

解释:
起始索引等于 0 的子串是 "ab", 它是 "ab" 的字母异位词。
起始索引等于 1 的子串是 "ba", 它是 "ab" 的字母异位词。
起始索引等于 2 的子串是 "ab", 它是 "ab" 的字母异位词。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-all-anagrams-in-a-string
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
此题和30题是基本是一样的
2. p中的字符串会重复
3. 使用map来对p中的每个字符串进行计数统计
4. 对s中的每个字符作为起始进行统计,如果能让map中计数降为0,则找到一个位置
*/
/*
问题:
一开始就认为坐标i,j的范围是0到s.len()-p.len(),这是错的,
i,j都应该走到最后
*/
use std::collections::HashMap;

struct Solution {}

#[derive(Default)]
struct Term {
    count: i32,
    // p中某个字符出现的次数
    expect: i32, //在匹配的过程中某个字符出现的次数
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
            count: 0,
            expect: 0,
        }
    }
    fn incCount(&mut self) {
        self.count += 1;
    }
    fn incExpect(&mut self) -> ExpectResult {
        self.expect += 1;
        if self.count > self.expect {
            return ExpectResult::Less;
        } else if self.count == self.expect {
            return ExpectResult::Equal;
        } else {
            return ExpectResult::Greater;
        }
    }
    fn dec_expect(&mut self) {
        self.expect -= 1;
    }
    fn reset(&mut self) {
        self.expect = 0;
    }
    fn is_match(&self) -> bool {
        return self.count == self.expect;
    }
}
use std::collections::hash_map::Entry;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut m = HashMap::new();
        let mut v = Vec::new();
        let mut total_count = p.len() as i32;
        let mut last_count = 0;

        for b in p {
            m.entry(b).or_insert(Term::new()).incCount();
        }
        fn reset(m: &mut HashMap<&u8, Term>, last_count: &mut i32) {
            *last_count = 0;
            m.values_mut().for_each(|v: &mut Term| {
                v.reset();
            });
        };
        let mut i = 0;
        while i < s.len() {
            println!("i={}", i);
            let e = m.get_mut(&s[i]);
            match e {
                None => {
                    //b在p中就不存在, 直接重来就行了
                    reset(&mut m, &mut last_count);
                    i += 1;
                    continue;
                }
                Some(_) => {
                    //从b开始循环匹配,直到全部匹配位置
                    let mut cnt = last_count;
                    for j in i..s.len() {
                        println!("j={}", j);
                        let e = m.get_mut(&s[j]);
                        match e {
                            None => {
                                //找到不在p中的字符,直接从头开始
                                reset(&mut m, &mut last_count);
                                i = j + 1;
                                break;
                            }
                            Some(mut e) => {
                                cnt += 1;
                                let r = e.incExpect();
                                //总数匹配了,并且最近一次数量刚好相等,说明找到了一个完整匹配
                                if r == ExpectResult::Greater {
                                    //有一个字符多了,从下一个开始匹配
                                    i = j + 1 - cnt as usize + 1; //这里不能是i=i+1,因为可能是上一次匹配从而导致i并没有指向起始
                                    reset(&mut m, &mut last_count);
                                    break;
                                }
                                if cnt == total_count && r == ExpectResult::Equal {
                                    let start = j + 1 - p.len();
                                    v.push(start as i32);
                                    //                                    e2.dec_expect();
                                    m.get_mut(&s[start]).expect("must exist").dec_expect();
                                    i = j + 1; //从当前位置继续往下匹配,只是把i从m中移除
                                    last_count = total_count - 1;
                                    println!("break to i={}", i);
                                    break; //只能从下一个开始匹配
                                }
                                //没有匹配完整的情形,继续
                            }
                        }
                    }
                    //走到最后了
                    //                    panic!(format!("i={}", i));
                }
            }
        }
        v
    }
    /**
    别人实现的既简单,清晰还高效
    这里主要是充分利用了和30题的不一样的地方
    1. p中的是一个个字符而不是字符串,所以不需要用map,直接用数组就ok了
    2. 既然是固定长度,那么每次全检查一遍,代价也很小,而我的解法为了避免重复检查,让下标不连续,导致代码无谓的复杂了.
    */
    pub fn find_anagrams2(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }
        let N = p.len();
        let mut map = [0; 26];
        let index_ch = |ch: char| ch as usize - 97;
        for ch in p.chars() {
            map[index_ch(ch)] += 1;
        }
        for ch in s.chars().take(N) {
            map[index_ch(ch)] -= 1;
        }
        let mut ans = vec![];
        let schars: Vec<char> = s.chars().collect();
        for i in 0..(s.len() - N) {
            if map.iter().all(|x| *x == 0) {
                ans.push(i as i32);
            }
            map[index_ch(schars[i])] += 1;
            map[index_ch(schars[i + N])] -= 1;
        }
        if map.iter().all(|x| *x == 0) {
            ans.push((s.len() - N) as i32);
        }
        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sorted_list_to_bst() {
        let t = Solution::find_anagrams(String::from("abab"), String::from("ab"));
        assert_eq!(t, vec![0, 1, 2]);
        let t = Solution::find_anagrams(String::from("abaacbabc"), String::from("abc"));
        assert_eq!(t, vec![3, 4, 6]);
    }
}
