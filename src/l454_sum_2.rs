/*
454. 四数相加 II

给定四个包含整数的数组列表 A , B , C , D ,计算有多少个元组 (i, j, k, l) ，使得 A[i] + B[j] + C[k] + D[l] = 0。

为了使问题简单化，所有的 A, B, C, D 具有相同的长度 N，且 0 ≤ N ≤ 500 。所有整数的范围在 -228 到 228 - 1 之间，最终结果不会超过 231 - 1 。

例如:

输入:
A = [ 1, 2]
B = [-2,-1]
C = [-1, 2]
D = [ 0, 2]

输出:
2

解释:
两个元组如下:
1. (0, 0, 0, 1) -> A[0] + B[0] + C[0] + D[1] = 1 + (-2) + (-1) + 2 = 0
2. (1, 1, 0, 0) -> A[1] + B[1] + C[0] + D[0] = 2 + (-1) + (-1) + 0 = 0

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/4sum-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路
分别对A,B和C,D之间进行累加,然后保存到统一的map中,然后计数
这样A+B=t.那么找-t就行了.
然后相乘即为计数
*/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut mapab = HashMap::new();
        let mut mapcd = HashMap::new();
        for i in 0..a.len() {
            for j in 0..b.len() {
                let t1 = a[i] + b[j];
                let t2 = c[i] + d[j];
                *mapab.entry(t1).or_insert(0) += 1;
                *mapcd.entry(t2).or_insert(0) += 1;
            }
        }
        let mut cnt = 0;
        for (k, v) in mapab {
            if let Some(t2) = mapcd.get(&(-k)) {
                cnt += v * t2;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::share;
    #[test]
    fn test_reverse_list() {
        let t = Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]);
        assert_eq!(t, 2);
    }
}
