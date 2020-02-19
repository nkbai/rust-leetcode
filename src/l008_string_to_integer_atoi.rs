/*
请你来实现一个 atoi 函数，使其能将字符串转换成整数。

首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。

当我们寻找到的第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字组合起来，作为该整数的正负号；假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成整数。

该字符串除了有效的整数部分之后也可能会存在多余的字符，这些字符可以被忽略，它们对于函数不应该造成影响。

注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，则你的函数不需要进行转换。

在任何情况下，若函数不能进行有效的转换时，请返回 0。

说明：

假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为 [−231,  231 − 1]。如果数值超过这个范围，qing返回  INT_MAX (231 − 1) 或 INT_MIN (−231) 。

示例 1:

输入: "42"
输出: 42
示例 2:

输入: "   -42"
输出: -42
解释: 第一个非空白字符为 '-', 它是一个负号。
     我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。
示例 3:

输入: "4193 with words"
输出: 4193
解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。
示例 4:

输入: "words and 987"
输出: 0
解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
     因此无法执行有效的转换。
示例 5:

输入: "-91283472332"
输出: -2147483648
解释: 数字 "-91283472332" 超过 32 位有符号整数范围。
     因此返回 INT_MIN (−231)
 */

struct Solution {}
impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        //移除前缀空格
        let s = str.as_bytes();

        //最终结果
        let mut result: i32 = 0;
        let mut sign_nagative: bool = false;
        let mut first_zero = true;
        let mut first_sign = true;
        let mut strip_space = true;
        if s.len() <= 0 {
            return 0;
        }
        println!("str={}", str);
        for i in 0..s.len() {
            if strip_space && s[i] == ' ' as u8 {
                continue; //跳过开始的空白
            }
            strip_space = false;
            if first_sign {
                //处理第一个符号或者数字
                let temp = s[i] as char;
                match temp {
                    '-' => sign_nagative = true,
                    '+' => sign_nagative = false,
                    n if n >= '0' && n <= '9' => result = temp as i32 - '0' as i32,
                    _ => return 0,
                }
                first_sign = false;
                first_zero = result == 0; //如果是0，表示后面还可以继续是0，不算数
                continue;
            }
            if first_zero && s[i] == '0' as u8 {
                continue; //跳过开始的0 -001
            }
            first_zero = false;
            if s[i] > '9' as u8 || s[i] < '0' as u8 {
                return result;
            }

            let pos = match sign_nagative {
                true => ('0' as i32) - s[i] as i32,
                false => s[i] as i32 - ('0' as i32),
            };

            let new = result as i64 * 10 + pos as i64;
            //向上溢出
            if new > core::i32::MAX as i64 {
                return core::i32::MAX;
            } else if new < core::i32::MIN as i64 {
                //向下溢出
                return core::i32::MIN;
            } else {
                result = new as i32;
            }
        }
        result
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_my_atoi() {
        assert_eq!(42, Solution::my_atoi(String::from("42")));
        assert_eq!(-42, Solution::my_atoi(String::from("    -42")));
        assert_eq!(4193, Solution::my_atoi(String::from("4193 with words")));
        assert_eq!(0, Solution::my_atoi(String::from("words and 987")));
        assert_eq!(1, Solution::my_atoi(String::from("+1")));
        assert_eq!(0, Solution::my_atoi(String::from("+-1")));
        assert_eq!(-2147483648, Solution::my_atoi(String::from("-91283472332")));
        assert_eq!(
            2147483647,
            Solution::my_atoi(String::from("20000000000000000000"))
        )
    }
}
