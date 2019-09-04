/*
925. 长按键入
你的朋友正在使用键盘输入他的名字 name。偶尔，在键入字符 c 时，按键可能会被长按，而字符可能被输入 1 次或多次。

你将会检查键盘输入的字符 typed。如果它对应的可能是你的朋友的名字（其中一些字符可能被长按），那么就返回 True。



示例 1：

输入：name = "alex", typed = "aaleex"
输出：true
解释：'alex' 中的 'a' 和 'e' 被长按。
示例 2：

输入：name = "saeed", typed = "ssaaedd"
输出：false
解释：'e' 一定需要被键入两次，但在 typed 的输出中不是这样。
示例 3：

输入：name = "leelee", typed = "lleeelee"
输出：true
示例 4：

输入：name = "laiden", typed = "laiden"
输出：true
解释：长按名字中的字符并不是必要的。


提示：

name.length <= 1000
typed.length <= 1000
name 和 typed 的字符都是小写字母。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/long-pressed-name
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
很简单,这个就是一个正则表达式匹配的问题.
因为涉及到匹配重复字符问题,碰到重复的字符,一次性吃下全部即可.
*/
struct Solution {}
impl Solution {
    fn get_next_bytes_len(name: &[u8], i: usize) -> usize {
        let a = name[i];
        for j in (i + 1)..name.len() {
            if name[j] != a {
                return j - i;
            }
        }
        name.len() - i //全部都一样了
    }
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name = name.as_bytes();
        let typed = typed.as_bytes();
        let mut ni = 0;
        let mut ti = 0;
        loop {
            //都走完了,匹配成功,结束
            if name.len() <= ni && typed.len() <= ti {
                return true;
            }
            let c = name[ni];
            let nl = Solution::get_next_bytes_len(name, ni);
            ni += nl; //ni移动
            for i in ti..(ti + nl) {
                if ti >= typed.len() || typed[i] != c {
                    return false; //必须匹配的,没有匹配上,失败
                }
            }
            //尽可能吃下所有的相同字符
            ti = ti + nl; //ti先移动相同的
            let range = ti..typed.len();
            for i in range {
                if typed[i] != c {
                    break;
                }
                ti += 1; //移动尽可能多的
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_long_pressed() {
        assert_eq!(
            Solution::is_long_pressed_name(String::from("saeed"), String::from("ssaaedd")),
            false
        );
        assert_eq!(
            Solution::is_long_pressed_name(String::from("leelee"), String::from("lleeelee")),
            true
        );
        assert_eq!(
            Solution::is_long_pressed_name(String::from("laiden"), String::from("laiden")),
            true
        );
        assert_eq!(
            Solution::is_long_pressed_name(String::from("alex"), String::from("aaleex")),
            true
        );
    }
}
