/*
1052. 爱生气的书店老板
今天，书店老板有一家店打算试营业 customers.length 分钟。每分钟都有一些顾客（customers[i]）会进入书店，所有这些顾客都会在那一分钟结束后离开。

在某些时候，书店老板会生气。 如果书店老板在第 i 分钟生气，那么 grumpy[i] = 1，否则 grumpy[i] = 0。 当书店老板生气时，那一分钟的顾客就会不满意，不生气则他们是满意的。

书店老板知道一个秘密技巧，能抑制自己的情绪，可以让自己连续 X 分钟不生气，但却只能使用一次。

请你返回这一天营业下来，最多有多少客户能够感到满意的数量。


示例：

输入：customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], X = 3
输出：16
解释：
书店老板在最后 3 分钟保持冷静。
感到满意的最大客户数量 = 1 + 1 + 1 + 1 + 7 + 5 = 16.


提示：

1 <= X <= customers.length == grumpy.length <= 20000
0 <= customers[i] <= 1000
0 <= grumpy[i] <= 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/grumpy-bookstore-owner
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
粗暴的方法是O(XN),优化的方法是
使用三倍的空间将复杂度降低到O(N)
分别是从左往右,按照书店老板是否生气求累加和
从右往左,按照书店老板是否生气求累加和
从左往右,不考虑书店老板是否生气求累加和
*/
struct Solution {}
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let x = x as usize;
        let mut left = Vec::new(); //从0到i的和
        let mut right = Vec::new(); //从i到最后的和
        let mut total = Vec::new();
        //求左边
        for i in 0..customers.len() {
            right.push(0);
            let mut last = 0;
            if i > 0 {
                last = left[i - 1];
            }
            if grumpy[i] == 0 {
                left.push(last + customers[i]);
            } else {
                left.push(last);
            }
        }

        for i in (0..customers.len()).rev() {
            let mut last = 0;
            if i < customers.len() - 1 {
                last = right[i + 1];
            }
            if grumpy[i] == 0 {
                right[i] = last + customers[i];
            } else {
                right[i] = last;
            }
        }
        //求全部
        for i in 0..customers.len() {
            right.push(0);
            let mut last = 0;
            if i > 0 {
                last = total[i - 1];
            }
            total.push(last + customers[i]);
        }
        let mut max = 0;
        for i in 0..customers.len() - x + 1 {
            let mut v1 = 0;
            let mut v2 = 0;
            if i > 0 {
                v1 = left[i - 1];
            }
            if i + x <= customers.len() - 1 {
                v2 = right[i + x];
            }
            v1 += total[i + x - 1] - total[i] + customers[i];
            if v1 + v2 > max {
                max = v1 + v2;
            }
        }
        max
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let t = Solution::max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3,
        );
        assert_eq!(t, 16);
    }
}
