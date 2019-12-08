/*
452. 用最少数量的箭引爆气球

在二维空间中有许多球形的气球。对于每个气球，提供的输入是水平方向上，气球直径的开始和结束坐标。由于它是水平的，所以y坐标并不重要，因此只要知道开始和结束的x坐标就足够了。开始坐标总是小于结束坐标。平面内最多存在104个气球。

一支弓箭可以沿着x轴从不同点完全垂直地射出。在坐标x处射出一支箭，若有一个气球的直径的开始和结束坐标为 xstart，xend， 且满足  xstart ≤ x ≤ xend，则该气球会被引爆。可以射出的弓箭的数量没有限制。 弓箭一旦被射出之后，可以无限地前进。我们想找到使得所有气球全部被引爆，所需的弓箭的最小数量。

Example:

输入:
[[10,16], [2,8], [1,6], [7,12]]

输出:
2

解释:
对于该样例，我们可以在x = 6（射爆[2,8],[1,6]两个气球）和 x = 11（射爆另外两个气球）。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-number-of-arrows-to-burst-balloons
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
1. 首先按照起始位置排序,
2. 贪心算法,尽可能的多吃进去
3. 进来一个会不断的缩减区域.
4. 如果新进来一个不能放到现在的区域就新开一个.
因为新来的要么和左边的合并 ,要么新开,
如果和左边邻居的合并,那么意味着最左边的一定会被踢出去.
比如
1           6
  2              8
     3         7
[1,6]进来会把区域限定为[1,6]
[2,8]进来则会吧区域限定为[2,6]
这时候[3,7]进来,肯定放不进[2,6],如果和[2,8]组合就会把[1,6]踢出去
这时候使用的箭的数量是不变的.
*/

use std::cmp::{max, min};

struct Solution {}
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 0 {
            return 0;
        }
        let mut points = points;
        points.sort();
        //        println!("points={:?}", points);
        let mut cnt = 1;
        let (mut start, mut end) = (points[0][0], points[0][1]);
        for i in 1..points.len() {
            let p = points.get(i).unwrap();
            if p[0] <= end {
                start = max(p[0], start);
                end = min(p[1], end);
                continue;
            } else {
                //放不进去这个区域了.
                start = p[0];
                end = p[1];
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let t =
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]);
        assert_eq!(t, 2);
    }
}
