/*
我们有一个由平面上的点组成的列表 points。需要从中找出 K 个距离原点 (0, 0) 最近的点。

（这里，平面上两点之间的距离是欧几里德距离。）

你可以按任何顺序返回答案。除了点坐标的顺序之外，答案确保是唯一的。



示例 1：

输入：points = [[1,3],[-2,2]], K = 1
输出：[[-2,2]]
解释：
(1, 3) 和原点之间的距离为 sqrt(10)，
(-2, 2) 和原点之间的距离为 sqrt(8)，
由于 sqrt(8) < sqrt(10)，(-2, 2) 离原点更近。
我们只需要距离原点最近的 K = 1 个点，所以答案就是 [[-2,2]]。
示例 2：

输入：points = [[3,3],[5,-1],[-2,4]], K = 2
输出：[[3,3],[-2,4]]
（答案 [[-2,4],[3,3]] 也会被接受。）
*/

/*
思路:
这个题目实际上是一个部分排序问题
使用部分快速排序
*/

struct Solution {}
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        if k > points.len() {
            return points;
        }
        let mut points = points;
        points.sort_by(|n1, n2| {
            let a = n1[0] * n1[0] + n1[1] * n1[1];
            let b = n2[0] * n2[0] + n2[1] * n2[1];
            a.cmp(&b)
        });
        points.as_slice()[0..(k as usize)].into()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let t = Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
        println!("t={:?}", t);
    }
}
