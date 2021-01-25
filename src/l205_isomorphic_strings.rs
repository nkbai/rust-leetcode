#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
205. 同构字符串
给定两个字符串 s 和 t，判断它们是否是同构的。

如果 s 中的字符可以被替换得到 t ，那么这两个字符串是同构的。

所有出现的字符都必须用另一个字符替换，同时保留字符的顺序。两个字符不能映射到同一个字符上，但字符可以映射自己本身。

示例 1:

输入: s = "egg", t = "add"
输出: true
示例 2:

输入: s = "foo", t = "bar"
输出: false
示例 3:

输入: s = "paper", t = "title"
输出: true
说明:
你可以假设 s 和 t 具有相同的长度。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/isomorphic-strings
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
逐一映射,如果出现冲突就返回false,
没有任何冲突,返回true
*/
struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        //用数组来替代map,因为明确知道数量. s->t
        let mut m = [0 as u8; 255];
        //t->s 的映射,必须是双向的才符合题意
        let mut m2 = [0 as u8; 255];
        for i in 0..s.len() {
            let si = s[i] as usize;
            let ti = t[i] as usize;
            if m[si] != 0 && m[si] != ti as u8 {
                return false;
            }
            if m2[ti] != 0 && m2[ti] != si as u8 {
                return false;
            }
            m[si] = ti as u8;
            m2[ti] = si as u8;
        }
        true
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::is_isomorphic("egg".into(), "add".into()), true);
        assert_eq!(Solution::is_isomorphic("foo".into(), "bar".into()), false);
        assert_eq!(Solution::is_isomorphic("ab".into(), "aa".into()), false);
    }
}
