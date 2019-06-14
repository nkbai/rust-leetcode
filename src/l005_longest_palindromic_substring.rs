/*
https://leetcode-cn.com/problems/longest-palindromic-substring/
给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。

示例 1：

输入: "babad"
输出: "bab"
注意: "aba" 也是一个有效答案。
示例 2：

输入: "cbbd"
输出: "bb"

*/
/*
思路:
怎么把规模大的问题化成规模小的问题进行解决
假设用m[i][j]表示从i到j是回文的长度
那么只有两种情况可以扩展出回文
m[i][j]是回文,当且仅当:
1. m[i][j-1]是回文,并且m[i][j-1]长度是1,并且m[j-1]==m[j]
2. m[i+1][j-1]是回文,并且m[i]==m[j]
遍历的过程中记一个最长字符串即可.
*/
struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 0 {
            return s;
        }
        let ss = s.as_bytes();
        let mut m = vec![vec![0; ss.len()]; ss.len()];

        //        for i in 0..m.len() {
        //            m[i] = Vec::with_capacity(s.len());
        //        }
        for i in 0..m.len() {
            m[i][i] = 1; //自身肯定是一个回文
        }
        let mut longest = &ss[0..1];
        //step
        for k in 1..s.len() {
            //元素下标
            for i in 0..s.len() {
                if (i + k) < s.len() && m[i][i + k - 1] == 1 && ss[i + k - 1] == ss[i + k] {
                    m[i][i + k] = 2;
                    if longest.len() <= k {
                        longest = &ss[i..(i + k + 1)]; //包含最后一位
                    }
                } else {
                    let s = i + 1;
                    let e = i + k - 1;
                    if i + 1 >= m.len() || i + k >= m.len() {
                        continue; //越界的不考虑
                    }
                    if m[i + 1][i + k - 1] > 0 && ss[i] == ss[i + k] {
                        m[i][i + k] = m[i + 1][i + k - 1] + 2; //向周边扩展了两位
                        if longest.len() <= k {
                            longest = &ss[i..(i + k + 1)]; //包含最后一位
                        }
                    }
                }
            }
        }
        String::from_utf8(Vec::from(longest)).ok().unwrap()
    }

    //leetcode最快解法
    //.0:该元素坐标,.1 相同数值截止坐标
    fn pre_prase(s: String) -> Vec<(usize, usize)> {
        let s = s.chars().into_iter().collect::<Vec<char>>(); //iter 转vector
        let mut result = Vec::new();
        let mut l = 0_usize;
        let mut r = 0_usize;

        loop {
            while l < s.len() && r < s.len() && s[l] == s[r] {
                r += 1;
            }

            if l > s.len() || r > s.len() {
                break;
            }

            result.push((l, r - 1));
            l = r;
            r = l + 1;
        }

        result
    }

    pub fn longest_palindrome2(s: String) -> String {
        let chars = s.chars().into_iter().collect::<Vec<char>>();
        let length = chars.len();
        //        println!("{:?}",chars);

        // 边界值的处理很垃圾。。。
        match length {
            0 => return String::from(""),
            1 => return s,
            2 => {
                if chars[0] == chars[1] {
                    return s;
                }
            }
            _ => (), //什么都不做
        };

        let mut cur_pos = (0_usize, 0_usize);
        let mut cur_len = 0_usize;
        let mut max_pos = (0_usize, 0_usize);
        let mut max_len = 0_usize;
        let poses = Solution::pre_prase(s.clone()).into_iter();
        println!("poses:{:?}", poses);
        for pos in poses {
            // 1 2 3 4 5 6
            //            i+=1;

            cur_pos = pos;
            cur_len = 0;

            //从左右两边,选一个短的,这样也保证了后面cur.pos.1+j,cur.pos.0-j都在有效范围之内
            let ml = if cur_pos.0 < length - cur_pos.1 - 1 {
                cur_pos.0
            } else {
                length - cur_pos.1 - 1
            };
            let mut ml = ml + 1;
            //            println!("cur_pos {:?}, mml {}",cur_pos,ml);
            //这个思路好处就是他并不是以字符为单位向左右展开,而是以字符串为单位向左右展开,节省了重复遍历的时间
            //相当于一种改进的从中间向两边扩展的情形
            for j in 0..ml {
                if chars[cur_pos.1 + j] == chars[cur_pos.0 - j] {
                    cur_len += 1;
                } else {
                    break;
                }
            }
            if 2 * cur_len + cur_pos.1 - cur_pos.0 + 1 > 2 * max_len + max_pos.1 - max_pos.0 + 1 {
                max_len = cur_len;
                max_pos = cur_pos;
            }
            //            println!("grow pos? {:?} len {}  {} > {} ",max_pos,max_len,cur_len + cur_pos.1 - cur_pos.0 + 1, max_len + max_pos.1 - max_pos.0 + 1);
        }

        //        println!("res pos {:?} len {} ",max_pos,max_len);
        let mut v = Vec::new();
        let left = max_pos.0 + 1 - max_len;
        let right = max_pos.1 + max_len - 1;
        //        println!("left {} right {}",left,right);
        for i in left..right + 1 {
            v.push(chars[i]); //有没有办法不用push,直接用chars的slice方式呢?
        }
        v.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome(String::from("babad")), "bab");
        assert_eq!(Solution::longest_palindrome(String::from("babba")), "abba");
        assert_eq!(Solution::longest_palindrome(String::from("abcdef")), "a");
        assert_eq!(Solution::longest_palindrome(String::from("")), "");
        assert_eq!(
            Solution::longest_palindrome2(String::from("abbbbbbbbbbbbbbbcd")),
            "bb"
        );
    }
}
