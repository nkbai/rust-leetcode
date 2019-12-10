/*
134. 加油站
在一条环路上有 N 个加油站，其中第 i 个加油站有汽油 gas[i] 升。

你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。你从其中的一个加油站出发，开始时油箱为空。

如果你可以绕环路行驶一周，则返回出发时加油站的编号，否则返回 -1。

说明:

如果题目有解，该答案即为唯一答案。
输入数组均为非空数组，且长度相同。
输入数组中的元素均为非负数。
示例 1:

输入:
gas  = [1,2,3,4,5]
cost = [3,4,5,1,2]

输出: 3

解释:
从 3 号加油站(索引为 3 处)出发，可获得 4 升汽油。此时油箱有 = 0 + 4 = 4 升汽油
开往 4 号加油站，此时油箱有 4 - 1 + 5 = 8 升汽油
开往 0 号加油站，此时油箱有 8 - 2 + 1 = 7 升汽油
开往 1 号加油站，此时油箱有 7 - 3 + 2 = 6 升汽油
开往 2 号加油站，此时油箱有 6 - 4 + 3 = 5 升汽油
开往 3 号加油站，你需要消耗 5 升汽油，正好足够你返回到 3 号加油站。
因此，3 可为起始索引。
示例 2:

输入:
gas  = [2,3,4]
cost = [3,4,3]

输出: -1

解释:
你不能从 0 号或 1 号加油站出发，因为没有足够的汽油可以让你行驶到下一个加油站。
我们从 2 号加油站出发，可以获得 4 升汽油。 此时油箱有 = 0 + 4 = 4 升汽油
开往 0 号加油站，此时油箱有 4 - 3 + 2 = 3 升汽油
开往 1 号加油站，此时油箱有 3 - 3 + 3 = 3 升汽油
你无法返回 2 号加油站，因为返程需要消耗 4 升汽油，但是你的油箱只有 3 升汽油。
因此，无论怎样，你都不可能绕环路行驶一周。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/gas-station
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

/*
思路:
注意**如果题目有解，该答案即为唯一答案**
从头遍历,记下gas,cost累积差值最大的位置
如果最终总的gas>=cost,那么这个差值最大的下一个位置就是出发的位置
1. 如果total_gas>=total_cost 只要起始位置合适,那么一定能走完回一圈
2. 找个这个位置(称之为c)可以做出发位置,因为他的当前gas>cost
3. 如果不从他出发,能走完一圈,那么肯定会剩油.也就是c+1能走完一圈,则c一定能.
这与题目中说的答案唯一性冲突
*/

struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sum_gas = 0;
        let mut sum_cost = 0;
        let mut max_diff = std::i32::MAX;
        let mut max_diff_pos = -1;
        for i in 0..gas.len() {
            sum_gas += gas[i];
            sum_cost += cost[i];
            if sum_gas - sum_cost < max_diff {
                max_diff = sum_gas - sum_cost;
                max_diff_pos = i as i32;
            }
        }
        if sum_gas < sum_cost {
            return -1;
        }
        (max_diff_pos + 1) % (gas.len() as i32)
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
        let gas = vec![4, 5, 1, 2, 3];
        let cost = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 0);
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(Solution::can_complete_circuit(gas, cost), -1);
    }
}
