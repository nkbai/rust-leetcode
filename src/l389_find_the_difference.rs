/*
389. 找不同

给定两个字符串 s 和 t，它们只包含小写字母。

字符串 t 由字符串 s 随机重排，然后在随机位置添加一个字母。

请找出在 t 中被添加的字母。



示例:

输入：
s = "abcd"
t = "abcde"

输出：
e

解释：
'e' 是那个被添加的字母。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-the-difference
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
数组即可处理,
统计个数,大于s中的个数,就是要找的
*/

struct Solution {}
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut arr = [0; 26];
        for i in s {
            arr[*i as usize - 'a' as usize] += 1;
        }
        for i in t {
            let index = *i as usize - 'a' as usize;
            arr[index] -= 1;
            if arr[index] < 0 {
                return *i as char;
            }
        }
        panic!("must found");
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::find_the_difference(String::from("abcd"), String::from("abcde")),
            'e'
        );
    }
}
