/*
476. 数字的补数
给定一个正整数，输出它的补数。补数是对该数的二进制表示取反。

注意:

给定的整数保证在32位带符号整数的范围内。
你可以假定二进制数不包含前导零位。
示例 1:

输入: 5
输出: 2
解释: 5的二进制表示为101（没有前导零位），其补数为010。所以你需要输出2。
示例 2:

输入: 1
输出: 0
解释: 1的二进制表示为1（没有前导零位），其补数为0。所以你需要输出0。
*/
/*


*/
struct Solution {}
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let zeros = num.leading_zeros();
        println!("zeros={}", zeros);
        let ones = 32 - zeros;
        let mut s = 0;
        let mut i = 0;
        while i < ones {
            s |= 1 << i;
            i += 1;
        }
        let mut num = !num;
        num &= s;
        num
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_complement(3), 0);
        assert_eq!(Solution::find_complement(5), 2);
        assert_eq!(Solution::find_complement(-1), 0);
    }
}
