/*
https://leetcode-cn.com/problems/decode-ways/

一条包含字母 A-Z 的消息通过以下方式进行了编码：

'A' -> 1
'B' -> 2
...
'Z' -> 26
给定一个只包含数字的非空字符串，请计算解码方法的总数。

示例 1:

输入: "12"
输出: 2
解释: 它可以解码为 "AB"（1 2）或者 "L"（12）。
示例 2:

输入: "226"
输出: 3
解释: 它可以解码为 "BZ" (2 26), "VF" (22 6), 或者 "BBF" (2 2 6) 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/decode-ways
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
因为根据编号,只有连续两位的并且不超过26的才是有效的编码,
长度为N的解码方法:
a[0...N-1]={a[0...N-2] +a[N-1]}+{a[0...N-3]+a[n-2,n-1]}
如果后一种情况a[n-2,n-1]范围在26之内的话.
有点类似于斐波那契数列求解的过程
*/

struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        Solution::decodings(&s.as_bytes().iter().map(|i| i - ('0' as u8)).collect())
    }
    fn decodings(v: &Vec<u8>) -> i32 {
        if v.len() <= 0 {
            return 0;
        }
        let mut vcnt = Vec::with_capacity(v.len());
        if v[0] == 0 {
            return 0;
        }
        vcnt.push(1); //只有一个字符的时候肯定是只有一种编码
        for i in 1..v.len() {
            let mut cnt = 0;
            if v[i] != 0 {
                //只要不是0的数字,都是有效的编码
                cnt = vcnt[i - 1]; //第一种分法
            }
            if i >= 1 {
                //考虑 i-1,i联合起来的情形
                let n = v[i - 1] * 10 + v[i];
                if (v[i - 1] == 1 || v[i - 1] == 2) && n <= 26 {
                    //有效编码
                    if i >= 2 {
                        cnt += vcnt[i - 2];
                    } else {
                        cnt += 1; //考虑到只有两个字符,这时候下标就出界了.
                    }
                } else if n == 0 {
                    return 0; //连续的两个数字0,是不可能被编码的,后续的也无效了
                } else {
                    //超过的,不能有效编码,但是输入是合法的.
                }
            }
            vcnt.push(cnt);
        }
        vcnt[v.len() - 1]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_decodings() {
        assert_eq!(2, Solution::num_decodings(String::from("12")));
        assert_eq!(0, Solution::num_decodings(String::from("")));
        assert_eq!(0, Solution::num_decodings(String::from("0")));
        assert_eq!(1, Solution::num_decodings(String::from("10")));
        assert_eq!(1, Solution::num_decodings(String::from("101")));
        assert_eq!(0, Solution::num_decodings(String::from("100")));
        assert_eq!(1, Solution::num_decodings(String::from("1")));
        assert_eq!(3, Solution::num_decodings(String::from("226")));
        assert_eq!(2, Solution::num_decodings(String::from("227")));
    }
}
