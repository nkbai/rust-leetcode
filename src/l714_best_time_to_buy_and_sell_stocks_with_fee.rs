/*
714. 买卖股票的最佳时机含手续费

给定一个整数数组 prices，其中第 i 个元素代表了第 i 天的股票价格 ；非负整数 fee 代表了交易股票的手续费用。

你可以无限次地完成交易，但是你每次交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。

返回获得利润的最大值。

示例 1:

输入: prices = [1, 3, 2, 8, 4, 9], fee = 2
输出: 8
解释: 能够达到的最大利润:
在此处买入 prices[0] = 1
在此处卖出 prices[3] = 8
在此处买入 prices[4] = 4
在此处卖出 prices[5] = 9
总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
注意:

0 < prices.length <= 50000.
0 < prices[i] < 50000.
0 <= fee < 50000.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee
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
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        if prices.len() <= 0 {
            return 0;
        }
        let mut hold = -prices[0]; //持有股票
        let mut cash = 0; // 不持有股票
        for i in 1..prices.len() {
            cash = max(cash, hold + prices[i] - fee); //第i天不持有股票不外乎两种情况,前一天没有持有股票,卖掉了前一天持有的股票
            hold = max(hold, cash - prices[i]); //当前持有股票,不外乎前一天持有股票,或者今天买入了
        }
        cash
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    }
}
