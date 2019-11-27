/*
84. 柱状图中最大的矩形
给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。

求在该柱状图中，能够勾勒出来的矩形的最大面积。



![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/histogram.png)

以上是柱状图的示例，其中每个柱子的宽度为 1，给定的高度为 [2,1,5,6,2,3]。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/histogram_area.png)



图中阴影部分为所能勾勒出的最大矩形面积，其面积为 10 个单位。



示例:

输入: [2,1,5,6,2,3]
输出: 10

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/largest-rectangle-in-histogram
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
考虑到矩形的面积是由最矮的柱子决定的,
那么只要记录[j,i]之间最矮的柱子,那么[j,i]的面积就决定了是(i-j+1)*min
*/
use std::cmp::{max, min};
struct Solution {}
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut lowest = Vec::new();
        heights.iter().for_each(|_| {
            lowest.push(0);
        });
        let mut max_area = 0;
        //lowest[j]表示从当前i到j的最矮的柱子高度
        for i in 0..heights.len() {
            lowest[i] = heights[i]; //从i到i的最矮柱子就是i本身
            max_area = max(max_area, heights[i]);
            for j in (0..i).rev() {
                lowest[j] = min(heights[i], lowest[j]); //[j,i]区间
                max_area = max(max_area, (i - j + 1) as i32 * lowest[j]);
            }
        }
        max_area
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
        assert_eq!(t, 10);
        let t = Solution::largest_rectangle_area(vec![2]);
        assert_eq!(t, 2);
        let t = Solution::largest_rectangle_area(vec![2, 1, 2]);
        assert_eq!(t, 3);
    }
}
