/*
365. 水壶问题
有两个容量分别为 x升 和 y升 的水壶以及无限多的水。请判断能否通过使用这两个水壶，从而可以得到恰好 z升 的水？

如果可以，最后请用以上水壶中的一或两个来盛放取得的 z升 水。

你允许：

装满任意一个水壶
清空任意一个水壶
从一个水壶向另外一个水壶倒水，直到装满或者倒空
示例 1: (From the famous "Die Hard" example)

输入: x = 3, y = 5, z = 4
输出: True
示例 2:

输入: x = 2, y = 6, z = 5
输出: False

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/water-and-jug-problem
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
如果ax+by=z有整数解,
有整数解时当且仅当z是x和y的最大公约数d的倍数。
[裴蜀定理](https://baike.baidu.com/item/%E8%A3%B4%E8%9C%80%E5%AE%9A%E7%90%86/5186593)
*/

use std::cmp::{max, min};

struct Solution {}
impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        z == 0 || ((x + y) >= z && (z % Self::gcd(x, y)) == 0)
    }
    fn gcd(a: i32, b: i32) -> i32 {
        let t1 = max(a, b);
        let t2 = min(a, b);
        let mut a = t1;
        let mut b = t2;
        while b != 0 {
            let t1 = a % b;
            a = b;
            b = t1;
        }
        a
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::can_measure_water(3, 5, 4), true);
        assert_eq!(Solution::can_measure_water(2, 6, 5), false);
        assert_eq!(Solution::can_measure_water(1, 2, 3), true);
    }
}
