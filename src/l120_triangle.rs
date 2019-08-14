/*
120. 三角形最小路径和
给定一个三角形，找出自顶向下的最小路径和。每一步只能移动到下一行中相邻的结点上。

例如，给定三角形：
```
[
     [2],
    [3,4],
   [6,5,7],
  [4,1,8,3]
]
```
自顶向下的最小路径和为 11（即，2 + 3 + 5 + 1 = 11）。

说明：

如果你可以只使用 O(n) 的额外空间（n 为三角形的总行数）来解决这个问题，那么你的算法会很加分。



来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/triangle
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
自底向上
先求最下面一行的最小路径,就是他自身.
然后依次求出倒数第二行的所有元素的最短路径.
依次类推,最终得出自顶向下的最短路径.
时间复杂度为O(N) N为所有元素的个数
空间复杂度为O(n) n为三角形总行数
*/
struct Solution {}
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut row = triangle.len();
        let mut path = triangle[row - 1].clone();
        while path.len() > 1 {
            row -= 1;
            let rv = triangle.get(row - 1).unwrap();
            for i in 0..row {
                let m = path[i].min(path[i + 1]); //保证不会越界的
                path[i] = m + rv[i];
            }
            path.pop(); //扔掉最后一个
        }
        return path[0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mininum_total() {
        assert_eq!(
            11,
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3],])
        )
    }
}
