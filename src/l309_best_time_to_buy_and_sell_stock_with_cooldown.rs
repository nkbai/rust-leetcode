/*
309. 最佳买卖股票时机含冷冻期

给定一个整数数组，其中第 i 个元素代表了第 i 天的股票价格 。​

设计一个算法计算出最大利润。在满足以下约束条件下，你可以尽可能地完成更多的交易（多次买卖一支股票）:

你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
卖出股票后，你无法在第二天买入股票 (即冷冻期为 1 天)。
示例:

输入: [1,2,3,0,2]
输出: 3
解释: 对应的交易状态为: [买入, 卖出, 冷冻期, 买入, 卖出]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-cooldown
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
所谓冷冻期,很简单,就是你买入的时候参考的应该是i-2不持有股票的状态,而不是i-1天
*/
struct Solution {}
use std::cmp::max;
impl Solution {
    //按照框架来写的方法
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 0 {
            return 0;
        }
        let mut dp = vec![vec![0; 2]; prices.len()];
        dp[0][0] = 0;
        dp[0][1] = -prices[0]; //初始不持有股票的利润是0,持有股票表示第一天买入了,那么他的利润就是负数
        for i in 1..prices.len() {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] + prices[i]); //第i天不持有股票不外乎两种情况,前一天没有持有股票,卖掉了前一天持有的股票
            let mut last = 0;
            if i >= 2 {
                last = dp[i - 2][0];
            }
            dp[i][1] = max(dp[i - 1][1], last - prices[i]); //当前持有股票,不外乎前一天持有股票,或者今天买入了,今天买入参考利润应该是前前天的
        }
        dp[prices.len() - 1][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }
}
