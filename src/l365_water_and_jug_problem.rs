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
思路:
暴力穷举,但是主要保存状态,对于明确不行的,中间过程每个状态都保存.
对于任意一个状态
对两个水壶,不外乎三种操作
1.加满
2.倒空
3.倒给对方
得到一个新的状态,
如果所有可能的路都走不通,那么这个状态也是无效的.
*/

use std::cmp::{max, min};
use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if x + y < z {
            return false;
        }
        if z == 0 && x == 0 && y == 0 {
            return true;
        }
        let mut state = HashSet::new();
        return Self::measure_internal((x, 0), (y, 0), z, &mut state);
    }
    fn measure_internal(
        x: (i32, i32),
        y: (i32, i32),
        z: i32,
        state: &mut HashSet<(i32, i32)>,
    ) -> bool {
        //已经测试过,不行
        if state.contains(&(x.1, y.1)) {
            return false;
        }
        println!("try {:?}", (x.1, y.1));
        //刚好合适
        if x.1 + y.1 == z {
            return true;
        }
        //不要重复自己了
        state.insert((x.1, y.1));
        //加满x
        if x.1 < x.0 {
            if Self::measure_internal((x.0, x.0), y, z, state) {
                return true;
            }
            //x满,y保持不变,行不通
            state.insert((x.0, y.1));
        }
        //倒空x
        if x.1 > 0 {
            if Self::measure_internal((x.0, 0), y, z, state) {
                return true;
            }
            state.insert((0, y.1));
        }
        //倒给y
        if x.1 > 0 && y.1 < y.0 {
            let y1 = min(x.1, y.0 - y.1);
            let x1 = max(0, x.1 - y1);
            if Self::measure_internal((x.0, x1), (y.0, y1), z, state) {
                return true;
            }
            state.insert((x1, y1));
        }
        //加满y
        if y.1 < y.0 {
            if Self::measure_internal(x, (y.0, y.0), z, state) {
                return true;
            }
            state.insert((x.1, y.0));
        }
        //倒空y
        if y.1 > 0 {
            if Self::measure_internal(x, (y.0, 0), z, state) {
                return true;
            }
            state.insert((x.1, 0));
        }
        //倒给x
        if y.1 > 0 && x.1 < x.0 {
            let x1 = min(y.1, x.0 - x.1);
            let y1 = max(0, y.1 - x1);
            if Self::measure_internal((x.0, x1), (y.0, y1), z, state) {
                return true;
            }
            state.insert((x1, y1));
        }

        false
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
