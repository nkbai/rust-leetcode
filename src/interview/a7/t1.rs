/*
给定一个整数数组 prices，其中第 i 个元素代表了第 i 天的股票价格 ；非负整数 fee 代表了交易股票的手续费用。

你可以无限次地完成交易，但是你每次交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。

返回获得利润的最大值。

示例 1:

输入: prices = [1, 3, 2, 8, 4, 9], fee = 2
输出: 8
解释: 能够达到的最大利润:
在此处买入 prices[0] = 1
在此处卖出 prices[3] = 8
在此处买入 prices[4] = 4
在此处卖出 prices[5] = 9
总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
注意:

0 < prices.length <= 50000.
0 < prices[i] < 50000.
0 <= fee < 50000.
*/
/*
思路1:
从前往后:
如果递增,则一直递增到最大,开始下降
这时候有个分叉,选择卖或者不卖
卖的话,计算剩下的收益加上这次卖出的
不卖的话,把当前最小值存下来,继续走下去
时间复杂度:O(2^N),肯定不可取
*/
struct Solution {}
use std::cmp::max;

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let prices = prices.as_slice();
        let mut v = Vec::new();
        return Self::profit(prices, fee, &mut v);
    }
    #[allow(dead_code)]
    fn profit(mut prices: &[i32], fee: i32, v: &mut Vec<i32>) -> i32 {
        // println!("prices={:?},v={:?}", prices, v);
        let mut _max_profit = 0;
        let mut _profit = 0;
        for i in 0..prices.len() {
            let cur = prices[i];
            if v.len() == 0 {
                v.push(cur);
                continue;
            }
            let last = *v.last().unwrap();
            if cur >= last {
                if cur > last {
                    let first = v.first().unwrap();
                    if cur > *first + fee {
                        v.push(cur);
                    } //这时候如果里面有多余的,应该都弹出来才对
                }
                //增长一点点的,直接丢弃了
                continue;
            } else {
                if v.len() == 1 {
                    v.pop();
                    v.push(cur);
                    continue;
                }
                //>=2 尝试分叉
                let first = *v.first().unwrap();

                let mut profit = last - first - fee; //肯定大于0.
                let mut v2 = Vec::new();
                v2.push(cur); //卖
                let mut v3 = Vec::new();
                v3.push(first); //不卖
                prices = &prices[i + 1..prices.len()];
                let p1 = Self::profit(prices, fee, &mut v2);
                let p2 = Self::profit(prices, fee, &mut v3);
                // println!("cur={},profit={},p1={},p2={}", cur, profit, p1, p2);
                profit = max(profit + p1, p2);
                return profit;
            }
        }
        if v.len() >= 2 {
            let first = *v.first().unwrap();
            let last = *v.last().unwrap();
            return last - first - fee;
        }
        0
    }
}
#[cfg(test)]
mod test {
    #![allow(unused_imports, dead_code)]
    use super::*;

    #[test]
    fn test() {}
}
