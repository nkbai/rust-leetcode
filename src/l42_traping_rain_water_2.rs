/*
给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。



上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 感谢 Marcos 贡献此图。
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/rainwatertrap.png)
示例:

输入: [0,1,0,2,1,0,1,3,2,1,2,1]
输出: 6
在真实的面试中遇到过这道题？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/trapping-rain-water
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
递减栈 (可以相等)
不用像上一种方法那么复杂
每次只计算增加的雨水量
以[5,2,1,2,1,5]
为例:
[5,2,1]
这时候来了2,那么增加了雨水是1
[5,2,2]->[5,2,2,1]
这时候来了5,
弹出1,增加(min(5,2)-1)*1
弹出2,增加(min(5,2)-2)*3
弹出2, 增加(min(5,5)-2)*4
*/
use std::cmp::min;

struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut total = 0;
        for i in 0..height.len() {
            //要把0也放进去,把0放进去,简化了很多问题
            while stack.len() > 0 && height[*stack.last().unwrap()] < height[i] {
                let pop_top = stack.pop().unwrap();
                if stack.is_empty() {
                    break;
                }
                let top = *stack.last().unwrap();
                let distance = i - top - 1;
                let height = min(height[i], height[top]) - height[pop_top];
                total += height * distance as i32
            }
            //            println!("push {}={}", i, height[i]);
            stack.push(i);
        }
        total
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let t = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(t, 6);
        let t = Solution::trap(vec![4, 2, 3]);
        assert_eq!(t, 1);
        let t = Solution::trap(vec![5, 2, 1, 2, 1, 5]);
        assert_eq!(t, 14);
    }
}
