/*
122. 买卖股票的最佳时机 II

给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。

设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。

注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

示例 1:

输入: [7,1,5,3,6,4]
输出: 7
解释: 在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6-3 = 3 。
示例 2:

输入: [1,2,3,4,5]
输出: 4
解释: 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
示例 3:

输入: [7,6,4,3,1]
输出: 0
解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
有一点肯定是真的:
我整体的最好收益取决于历史收益+未来收益
在当前状态: 我的最好收益不外乎两种情况,持有股票或者不持有股票两种情况
那么明天我的状态就是基于这两种情况向前继续滚动
交易次数不限制,那就不用关心买卖次数
dp[i][k][state]
state:0 表示持有股票,1表示不持有股票 dp里面存储的是这个状态对应的利润
https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/solution/yi-ge-fang-fa-tuan-mie-6-dao-gu-piao-wen-ti-by-l-2/
*/
struct Solution {}
use std::cmp::max;
impl Solution {
    //因为只有前后相关性,所以用不着存储i-2的状态
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 0 {
            return 0;
        }
        let mut hold = -prices[0]; //持有股票
        let mut cash = 0; // 不持有股票
        for i in 1..prices.len() {
            cash = max(cash, hold + prices[i]); //第i天不持有股票不外乎两种情况,前一天没有持有股票,卖掉了前一天持有的股票
            hold = max(hold, cash - prices[i]); //当前持有股票,不外乎前一天持有股票,或者今天买入了
        }
        cash
    }
    //按照框架来写的方法
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        if prices.len() <= 0 {
            return 0;
        }
        let mut dp = vec![vec![0; 2]; prices.len()];
        dp[0][0] = 0;
        dp[0][1] = -prices[0]; //初始不持有股票的利润是0,持有股票表示第一天买入了,那么他的利润就是负数
        for i in 1..prices.len() {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] + prices[i]); //第i天不持有股票不外乎两种情况,前一天没有持有股票,卖掉了前一天持有的股票
            dp[i][1] = max(dp[i - 1][1], dp[i - 1][0] - prices[i]); //当前持有股票,不外乎前一天持有股票,或者今天买入了
        }
        dp[prices.len() - 1][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }
}
