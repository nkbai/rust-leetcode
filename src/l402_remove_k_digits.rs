/*
402. 移掉K位数字
给定一个以字符串表示的非负整数 num，移除这个数中的 k 位数字，使得剩下的数字最小。

注意:

num 的长度小于 10002 且 ≥ k。
num 不会包含任何前导零。
示例 1 :

输入: num = "1432219", k = 3
输出: "1219"
解释: 移除掉三个数字 4, 3, 和 2 形成一个新的最小的数字 1219。
示例 2 :

输入: num = "10200", k = 1
输出: "200"
解释: 移掉首位的 1 剩下的数字为 200. 注意输出不能有任何前导零。
示例 3 :

输入: num = "10", k = 2
输出: "0"
解释: 从原数字移除所有的数字，剩余为空就是0。


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/remove-k-digits
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
贪心算法,每次移除一个,都让其最小,这样可以保证连续移除多位后,仍然是最小.
 移除权重尽可能大的那个位置
比如1432219 ,为什么先移除4而不移除9,4虽然小于9,但是其权重大
假设n1=4,位置=1,n2=9,位置=6 那么n2的相比较权重是,n1的则是4*10^(6-1-1)
因此移除4而不是移除9
*/

struct Solution;
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut num = num;
        if k <= 0 {
            return num;
        }
        let mut sel_pos = 0;
        let bytes = num.as_bytes();
        let mut sel_num = bytes[0];
        //移除一个让这个字符串尽可能小的数字
        for i in 1..bytes.len() {
            if i - sel_pos == 1 && bytes[i] >= sel_num {
                sel_pos = i;
                sel_num = bytes[i];
            }
        }
        num.remove(sel_pos);
        while num.len() > 0 {
            if num.as_bytes()[0] != '0' as u8 {
                break;
            }
            num.remove(0);
        }
        //除非没有可移除的,否则都应该继续
        if num.len() == 0 {
            return String::from("0");
        }
        return Solution::remove_kdigits(num, k - 1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_kdigits() {
        assert_eq!("1219", Solution::remove_kdigits(String::from("1432219"), 3));
        assert_eq!("200", Solution::remove_kdigits(String::from("10200"), 1));
        assert_eq!("0", Solution::remove_kdigits(String::from("10"), 2));
        assert_eq!("11", Solution::remove_kdigits(String::from("112"), 1));
    }
}
