/*
386. 字典序排数
给定一个整数 n, 返回从 1 到 n 的字典顺序。

例如，

给定 n =1 3，返回 [1,10,11,12,13,2,3,4,5,6,7,8,9] 。

请尽可能的优化算法的时间复杂度和空间复杂度。 输入的数据 n 小于等于 5,000,000。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/lexicographical-numbers
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
把数据看成树状结构就很容易了.
一开始是root 有1,3,...9 九个节点
1 有十个节点10,11,...,19
10 有十个子节点100,101,102,... 109
11 有十个节点110,111,112,...,119
依次类推,因此直接使用DFS遍历就ok了
需要使用的空间是logN,时间复杂度就是O(N)
*/
struct Solution {}
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        if n <= 0 {
            panic!("n={} is impossible", n);
        }
        let mut res = Vec::with_capacity(n as usize);
        for c in 1..10 {
            Self::order(c, n, &mut res);
        }
        res
    }
    fn order(c: i32, n: i32, res: &mut Vec<i32>) {
        if c > n {
            return;
        }
        res.push(c);
        for i in 0..10 {
            Self::order(c * 10 + i, n, res);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let v = vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(v, Solution::lexical_order(13));
        let v = vec![1];
        assert_eq!(v, Solution::lexical_order(1));
        let v = vec![1, 2];
        assert_eq!(v, Solution::lexical_order(2));
    }
}
