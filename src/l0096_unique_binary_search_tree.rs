/*
https://leetcode-cn.com/problems/unique-binary-search-trees/
给定一个整数 n，求以 1 ... n 为节点组成的二叉搜索树有多少种？

示例:

输入: 3
输出: 5
解释:
给定 n = 3, 一共有 5 种不同结构的二叉搜索树:

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3
*/

/*
思路:
搜索树的特点就是左小右大
因此:
这是一个纯计算的过程,都不用模拟二叉树
1...n
G[i]表示有i个节点的二叉树有多少种
G[N]=G[0]*G[N-1]+G[1]*G[N-2]+G[2]*[N-3]+....
考虑1做定点的情况主要考虑右边N-1个节点的各种组合怎么挂在1的右子树
考虑2做顶点的情况,左边是一个节点的子树,右边是n-2个节点的子树
怎么把右边n-2个节点构成的子树挂到左边2个节点构成的各种子树的下面,作为其右子树
*/
struct Solution {}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut v = vec![0; (n + 1) as usize];
        v[0] = 1;
        v[1] = 1;
        return Solution::num_trees_internal(n as usize, &mut v);
    }
    pub fn num_trees_internal(n: usize, v: &mut Vec<i32>) -> i32 {
        if v[n] > 0 {
            return v[n];
        }
        let mut sum = 0;
        for i in 0..n {
            v[i] = Solution::num_trees_internal(i, v);
            v[n - i - 1] = Solution::num_trees_internal(n - i - 1, v);
            sum += v[i] * v[n - i - 1];
        }
        sum
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_num_trees() {
        assert_eq!(1, Solution::num_trees(1));
        assert_eq!(2, Solution::num_trees(2));
        assert_eq!(5, Solution::num_trees(3));
    }
}
