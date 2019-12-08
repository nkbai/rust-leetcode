/*
188. 买卖股票的最佳时机 IV
给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。

设计一个算法来计算你所能获取的最大利润。你最多可以完成 k 笔交易。

注意: 你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

示例 1:

输入: [2,4,1], k = 2
输出: 2
解释: 在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。
示例 2:

输入: [3,2,6,5,0,3], k = 2
输出: 7
解释: 在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。
在真实的面试中遇到

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iv
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
有一点肯定是真的:
我整体的最好收益取决于历史收益+未来收益
在当前状态: 我的最好收益不外乎两种情况,持有股票或者不持有股票两种情况
那么明天我的状态就是基于这两种情况向前继续滚动
交易次数不限制,那就不用关心买卖次数
dp[i][k][state]
state:0 表示持有股票,1表示不持有股票 dp里面存储的是这个状态对应的利润
https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/solution/yi-ge-fang-fa-tuan-mie-6-dao-gu-piao-wen-ti-by-l-2/

dp[i][k][0] 表示还剩下k次交易,不持有股票的利润
dp[i][k][1] 表示还剩下k次交易,持有股票的利润

时间复杂度分析
O(KN) k<=N
空间复杂度:
O(K)

*/
struct Solution {}
use std::cmp::max;
impl Solution {
    //k 很大的时候
    fn simple_max_profit(prices: &Vec<i32>) -> i32 {
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
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        println!("k={},priceslen={}", k, prices.len());
        if k == 0 || prices.len() == 0 {
            return 0;
        }
        let mut k = k as usize;
        if k >= prices.len() {
            return Self::simple_max_profit(&prices);
        }
        let prices = Self::remove_inscrease_order(prices);
        let mut dp = vec![vec![0; 2]; k as usize];
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
    //没必要动态分配那么多,因为至于上一个状态有关
    pub fn max_profit3(k: i32, prices: Vec<i32>) -> i32 {
        if k == 0 || prices.len() == 0 {
            return 0;
        }
        let mut k = k as usize;
        if k >= prices.len() / 2 + 1 {
            k = prices.len() / 2 + 1; //最多进行prices.len()/2次交易,不可能进行更多了,一买一卖
        }
        let mut dp = vec![vec![0; 2]; k as usize];

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
    //内存占用超限
    pub fn max_profit2(k: i32, prices: Vec<i32>) -> i32 {
        if k == 0 || prices.len() == 0 {
            return 0;
        }
        let mut k = k as usize;
        if k >= prices.len() / 2 + 1 {
            //这可以优化成不限制次数,退化成了O(N)
            k = prices.len() / 2 + 1; //最多进行prices.len()/2次交易,不可能进行更多了,一买一卖
        }
        let mut dp = vec![vec![vec![0; 2]; k as usize]; prices.len()];

        println!("k={},priceslen={}", k, prices.len());

        for i in 0..k {
            dp[0][i][1] = -prices[0]; //剩下k-1次交易,持有股票 ,
        }
        //        println!("dp0={:?}", dp[0]);
        for i in 1..prices.len() {
            let price = prices[i];
            for j in 0..k as usize {
                //剩下j次交易,不持有股票,
                dp[i][j][0] = max(dp[i - 1][j][0], dp[i - 1][j][1] + price);
                let mut last = 0;
                if j < k - 1 {
                    last = dp[i - 1][j + 1][0];
                }
                //剩下j次交易,持有股票
                dp[i][j][1] = max(dp[i - 1][j][1], last - price);
            }
            println!("i={}", i);
            //            println!("dp[{}]={:?}", i, dp[i]);
        }
        let l = prices.len();
        dp[l - 1][0][0]
        //        let mut m = 0;
        //        for i in 0..k {
        //            println!("i={}", dp[l - 1][i][0]);
        //            m = max(m, dp[l - 1][i][0]);
        //        }
        //        m
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // [3,2,6,5,0,3], k = 2
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3, 2, 3]), 7);
    }
}
