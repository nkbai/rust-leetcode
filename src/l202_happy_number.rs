#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
202. 快乐数

编写一个算法来判断一个数是不是“快乐数”。

一个“快乐数”定义为：对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和，然后重复这个过程直到这个数变为 1，也可能是无限循环但始终变不到 1。如果可以变为 1，那么这个数就是快乐数。

示例:

输入: 19
输出: true
解释:
12 + 92 = 82
82 + 22 = 68
62 + 82 = 100
12 + 02 + 02 = 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/happy-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

 */

/*
思路:
在求平方和的过程中如果碰到重复,并且重复的值不是1,则不是快乐数.
如果是1,自然是快乐数.

问题?
如何证明一定会出现循环呢?
一个简单的推论,不会出现无限循环不重复的情况,因为整数的范围是有限的,一定会出现重复.

如何判断重复没有呢?
1. set
2. 快慢指针
*/
struct Solution {}
impl Solution {
    fn bit_square(mut n: i32) -> i32 {
        let mut s = 0;
        while n != 0 {
            let i = n % 10;
            s += i * i;
            n /= 10;
        }
        s
    }
    pub fn is_happy(n: i32) -> bool {
        let mut fast = n;
        let mut slow = n;
        loop {
            slow = Self::bit_square(slow);
            fast = Self::bit_square(fast);
            fast = Self::bit_square(fast);
            if slow == fast {
                break;
            }
        }
        slow == 1
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::is_happy(19);
        assert_eq!(t, true);
    }
}
