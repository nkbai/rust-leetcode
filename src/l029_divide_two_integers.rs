/*
29. 两数相除
https://leetcode-cn.com/problems/divide-two-integers/
给定两个整数，被除数 dividend 和除数 divisor。将两数相除，要求不使用乘法、除法和 mod 运算符。

返回被除数 dividend 除以除数 divisor 得到的商。

示例 1:

输入: dividend = 10, divisor = 3
输出: 3
示例 2:

输入: dividend = 7, divisor = -3
输出: -2
说明:

被除数和除数均为 32 位有符号整数。
除数不为 0。
假设我们的环境只能存储 32 位有符号整数，其数值范围是 [−231,  231 − 1]。本题中，如果除法结果溢出，则返回 231 − 1。


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/divide-two-integers
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
1. 首先两个数都取绝对值,比如被除数为23,除数为3
2. 首先不断对除数左移1位,直到超过被除数,然后记下来临界值 比如左移3次超过,那么就说明被除数在3*2^2~3*2^3之间
3. 然后减去3*2^2,继续处理,知道被除数小于除数为止
*/
struct Solution {}
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut is_neg = false;
        if dividend ^ divisor < 0 {
            is_neg = true; //符号相反
        }
        let dividend = if dividend < 0 {
            0 - dividend as i64
        } else {
            dividend as i64
        };
        let divisor = if divisor < 0 {
            0 - divisor as i64
        } else {
            divisor as i64
        };
        let cnt = if divisor == 1 {
            dividend as i64
        } else {
            Solution::divide_internal(dividend as i64, divisor as i64)
        };
        if cnt >= std::i32::MAX as i64 {
            if is_neg {
                return std::i32::MIN;
            } else {
                return std::i32::MAX;
            }
        }
        if is_neg {
            return (0 - cnt) as i32;
        } else {
            return cnt as i32;
        }
    }
    fn divide_internal(dividend: i64, divisor: i64) -> i64 {
        //小于被除数,就返回
        if dividend < divisor {
            return 0;
        }
        let mut cnt = 1;
        let mut c = divisor;
        let mut last = 0;
        loop {
            if c >= dividend {
                break;
            }
            last = c;
            c = c << 1; //每次左移1
            cnt *= 2; // 左移一次就是两倍
        }
        //c--dividend--last
        let left = c - dividend;
        let right = dividend - last;
        if left == 0 {
            //不可能等于右边的
            return cnt; //刚好等于左边,
        }

        let div = Solution::divide_internal(right, divisor);
        return cnt / 2 + div;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_divide() {
        assert_eq!(1, Solution::divide(4, 3));
        assert_eq!(-1, Solution::divide(4, -3));
        assert_eq!(3, Solution::divide(10, 3));
        assert_eq!(2147483647, Solution::divide(-2147483648, -1));
        assert_eq!(-2147483648, Solution::divide(-2147483648, 1));
        assert_eq!(12, Solution::divide(1021989372, 82778243));
        assert_eq!(3, Solution::divide(302462516, 82778243));
    }
}
