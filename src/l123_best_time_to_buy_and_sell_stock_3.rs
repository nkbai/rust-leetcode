/*
123. 买卖股票的最佳时机 III
给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。

设计一个算法来计算你所能获取的最大利润。你最多可以完成 两笔 交易。

注意: 你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

示例 1:

输入: [3,3,5,0,0,3,1,4]
输出: 6
解释: 在第 4 天（股票价格 = 0）的时候买入，在第 6 天（股票价格 = 3）的时候卖出，这笔交易所能获得利润 = 3-0 = 3 。
随后，在第 7 天（股票价格 = 1）的时候买入，在第 8 天 （股票价格 = 4）的时候卖出，这笔交易所能获得利润 = 4-1 = 3 。
示例 2:

输入: [1,2,3,4,5]
输出: 4
解释: 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
示例 3:

输入: [7,6,4,3,1]
输出: 0
解释: 在这个情况下, 没有交易完成, 所以最大利润为 0。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
参考188,就是188的一个特例

 */
struct Solution {}

use std::cmp::max;
impl Solution {
    //考虑到1,5,7,30这样的序列,中间是不可能卖出的,可以移除大部分值
    fn remove_inscrease_order(prices: Vec<i32>) -> Vec<i32> {
        if prices.len() <= 0 {
            return prices;
        }
        let mut res = Vec::new();
        let mut cur_min = prices[0];
        let mut cur_max = prices[0];
        res.push(cur_min);
        for i in 1..prices.len() {
            let p = prices[i];
            if p < cur_max {
                //碰到了新一个由低到高
                if cur_max > cur_min {
                    res.push(cur_max);
                }
                res.push(p);
                cur_min = p;
            }
            cur_max = p;
        }
        if cur_max > cur_min {
            res.push(cur_max);
        }
        res
    }
    /*
    终极优化问题
    1. 如果k>prices.len,那么就退化成了不限制次数,
    2. 考虑到1,5,7,30这样的序列,中间是不可能卖出的,可以移除大部分值
    */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        println!("priceslen={}", prices.len());
        if prices.len() == 0 {
            return 0;
        }
        let mut k = 2;
        let prices = Self::remove_inscrease_order(prices);
        let mut dp = vec![vec![0; 2]; k];
        println!("k={},priceslen={}", k, prices.len());
        for i in 0..k {
            dp[i][1] = -prices[0]; //剩下k-1次交易,持有股票 ,
        }
        //        println!("dp0={:?}", dp[0]);
        for i in 1..prices.len() {
            let price = prices[i];
            for j in 0..k as usize {
                //剩下j次交易,不持有股票,
                dp[j][0] = max(dp[j][0], dp[j][1] + price);
                let mut last = 0;
                if j < k - 1 {
                    last = dp[j + 1][0];
                }
                //剩下j次交易,持有股票
                dp[j][1] = max(dp[j][1], last - price);
            }
            println!("i={}", i);
            //            println!("dp[{}]={:?}", i, dp[i]);
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }
}
