/*
将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。

比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：

L   C   I   R
E T O E S I I G
E   D   H   N
之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。

请你实现这个将字符串进行指定行数变换的函数：

string convert(string s, int numRows);
示例 1:

输入: s = "LEETCODEISHIRING", numRows = 3
输出: "LCIRETOESIIGEDHN"
示例 2:

输入: s = "LEETCODEISHIRING", numRows = 4
输出: "LDREOEIIECIHNTSG"
解释:
0     2n-2  4n-4
L     D     R
E   O E   I I
E C   I H   N
T     S     G
n-1  3n-3   5n-5
*/
struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let lens = s.len();
        let s = s.as_bytes();
        let mut s2 = String::new();
        let n = num_rows as usize;
        let t = n - 1;
        //一行为单位向下走,计算每列的位置
        for r in 0..n {
            //最多也就是n列了,
            for c in 0..(lens / n + 1) {
                if r == 0 {
                    //0,2n-2,4n-4
                    let i = c * 2 * (n - 1);
                    if i >= lens {
                        continue;
                    }
                    s2.push(s[i] as char);
                } else if r == t {
                    //n-1 3n-3 5n-5
                    let i = (c * 2 + 1) * (n - 1);
                    if i >= lens {
                        continue;
                    }
                    s2.push(s[i] as char);
                } else {
                    let first = c * 2 * (n - 1) + r;
                    let last = (c + 1) * 2 * (n - 1) - r;
                    //                    println!("first={},last={}", first, last);
                    if first >= lens {
                        continue;
                    }
                    s2.push(s[first] as char);
                    if last >= lens {
                        continue;
                    }
                    s2.push(s[last] as char);
                    //每行有两个,注意先后顺序
                }
            }
        }
        s2
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_conversion() {
        assert_eq!(
            "LCIRETOESIIGEDHN",
            Solution::convert(String::from("LEETCODEISHIRING"), 3)
        );
        assert_eq!(
            "LDREOEIIECIHNTSG",
            Solution::convert(String::from("LEETCODEISHIRING"), 4)
        );
        assert_eq!(
            "LEETCODEISHIRING",
            Solution::convert(String::from("LEETCODEISHIRING"), 1)
        );
    }
}
