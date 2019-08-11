/*
https://leetcode-cn.com/problems/longest-valid-parentheses/

32. 最长有效括号

给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。

示例 1:

输入: "(()"
输出: 2
解释: 最长有效括号子串为 "()"
示例 2:

输入: ")()())"
输出: 4
解释: 最长有效括号子串为 "()()"

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-valid-parentheses
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


*/

/*
思路:
A是一个有效的括号子串,B也是一个有效的括号子串,那么要么A,B互不相交,要么A包含在B中,或者B包含在A中
设计一个map
m[i]=j 表示从下标i到j是一个可以扩展到的最长字符串 i<j
同样m[j]=i表示下标j可以扩展到j的最长字符串 j>i

从第一个字符串开始碰到如果恰好i,i+1是一对,那么从这个开始向左右扩展,直到不能继续扩展为止
然后再m中记下扩展到的极限假设是i,i+3,
那么下一次就从i+4开始匹配

时间复杂度:
o(n) 字符串的长度遍历一遍即可
空间复杂度: 最糟糕的情况是()(()(()(()(() 基本上也是最多2n项在map中

优化空间:
感觉m没有存在的必要,因为如果左边可以扩展,肯定是紧挨着自己的,所以直接用上一个from,end判断即可.
*/
struct Solution {}

use std::collections::HashMap;
impl Solution {
    const LEFT_PARENTHESES: u8 = '(' as u8;
    const RIGHT_PARENTHESES: u8 = ')' as u8;
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut l = 0;
        let s = s.as_bytes();
        let mut i = 0;
        let mut m = HashMap::new();
        while i + 1 < s.len() {
            if s[i] == Solution::LEFT_PARENTHESES && s[i + 1] == Solution::RIGHT_PARENTHESES {
                let (from, end) = Solution::longest_internal(&mut m, s, i);
                m.insert(from, end);
                m.insert(end, from);
                if l < end - from + 1 {
                    l = end - from + 1;
                }
                i = end + 1;
                continue;
            }
            i += 1;
        }
        l as i32
    }
    //传入from表示碰到的一个()的左括号的下标,返回这次找到的最长字符串长度以及起始以及结束下标
    fn longest_internal(m: &mut HashMap<usize, usize>, s: &[u8], from: usize) -> (usize, usize) {
        let mut from = from;
        let mut end = from + 1; //确认end是)
        loop {
            if from >= 1 {
                if m.get(&(from - 1)).is_some() {
                    //from直接可以向左扩展到m[from-1],m[from-1]肯定小于from
                    from = m[&(from - 1)];
                    continue;
                } else {
                    //尝试向左右扩展
                    if end + 1 >= s.len() {
                        break;
                    }
                    if s[from - 1] == Solution::LEFT_PARENTHESES
                        && s[end + 1] == Solution::RIGHT_PARENTHESES
                    {
                        from = from - 1;
                        end = end + 1;
                        continue;
                    } //没有匹配到,尝试向后走两个字符匹配()
                    if end + 2 < s.len() {
                        if s[end + 1] == Solution::LEFT_PARENTHESES
                            && s[end + 2] == Solution::RIGHT_PARENTHESES
                        {
                            end += 2;
                            continue;
                        }
                    }
                    break;
                }
            } else if end + 2 < s.len() {
                if s[end + 1] == Solution::LEFT_PARENTHESES
                    && s[end + 2] == Solution::RIGHT_PARENTHESES
                {
                    end += 2;
                    continue;
                }
                break; //向右扩不会碰到已经知道的有效括号子串
            } else {
                break;
            }
        }
        (from, end)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(2, Solution::longest_valid_parentheses(String::from("(()")));
        assert_eq!(
            4,
            Solution::longest_valid_parentheses(String::from(")()())"))
        );
        assert_eq!(
            12,
            Solution::longest_valid_parentheses(String::from("(()()(()()))"))
        );
        assert_eq!(
            12,
            Solution::longest_valid_parentheses(String::from("((()()(()()))"))
        );
        assert_eq!(
            12,
            Solution::longest_valid_parentheses(String::from("(()()(()())))"))
        );
        assert_eq!(
            4,
            Solution::longest_valid_parentheses(String::from("(()))())("))
        );
    }
}
