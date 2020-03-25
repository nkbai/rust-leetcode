#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
299. 猜数字游戏
你正在和你的朋友玩 猜数字（Bulls and Cows）游戏：你写下一个数字让你的朋友猜。每次他猜测后，你给他一个提示，告诉他有多少位数字和确切位置都猜对了（称为“Bulls”, 公牛），有多少位数字猜对了但是位置不对（称为“Cows”, 奶牛）。你的朋友将会根据提示继续猜，直到猜出秘密数字。

请写出一个根据秘密数字和朋友的猜测数返回提示的函数，用 A 表示公牛，用 B 表示奶牛。

请注意秘密数字和朋友的猜测数都可能含有重复数字。

示例 1:

输入: secret = "1807", guess = "7810"

输出: "1A3B"

解释: 1 公牛和 3 奶牛。公牛是 8，奶牛是 0, 1 和 7。
示例 2:

输入: secret = "1123", guess = "0111"

输出: "1A1B"

解释: 朋友猜测数中的第一个 1 是公牛，第二个或第三个 1 可被视为奶牛。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/bulls-and-cows
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
先统计secret每个数字出现的次数,
然后处理guess,如果和secret相应位置相同,则a+1,同时刚刚统计的次数-1
如果不同,如果相应的数字统计次数大于0,则统计次数-1,然后b+1
a表示公牛的个数,b表示奶牛的个数
*/
struct Solution {}
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let secret = secret.as_bytes();
        let guess = guess.as_bytes();
        let mut stats = [0; 10]; //0-9出现的次数
        for c in secret {
            let c = *c - '0' as u8;
            stats[c as usize] += 1;
        }
        let mut a = 0;
        let mut b = 0;
        for (i, c) in guess.iter().enumerate() {
            let p = (*c - '0' as u8) as usize;
            if *c == secret[i] {
                a += 1;
                if stats[p] > 0 {
                    stats[p] -= 1;
                } else {
                    b -= 1; //1122,1222这种情形,先把2挪用给了B
                }
            } else {
                if stats[p] > 0 {
                    stats[p] -= 1;
                    b += 1;
                }
            }
        }
        format!("{}A{}B", a, b)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::get_hint("1807".into(), "7810".into());
        assert_eq!(t, "1A3B".to_string());
        let t = Solution::get_hint("1123".into(), "0111".into());
        assert_eq!(t, "1A1B".to_string());
        let t = Solution::get_hint("1122".into(), "1222".into());
        assert_eq!(t, "3A0B".to_string());
    }
}
