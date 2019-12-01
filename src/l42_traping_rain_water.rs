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
递减栈
栈内为(height,下标,已经计算的雨水面积,柱子的面积)

碰到j,需要弹出栈内的元素时,计算一下这次弹出的能承接的雨水量
如果弹出后,栈空了,那么计入最终总量,否则就计入栈顶元素.
*/
struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut total = 0;
        for (i, h) in height.iter().enumerate() {
            if *h == 0 {
                continue;
            }
            if stack.len() <= 0 {
                stack.push((*h, i, 0, 0)); //height,下标,左边雨水的面积,左边柱子的面积
                continue;
            }
            let mut c = (*h, i, 0, 0);
            let mut top = *stack.last().unwrap();
            /*
            top保存的是已经计算的雨水的面积,柱子的面积
            来了一个新的高度,需要考虑两部分
            1. top 到current之间的步伐
            2. top上已经计算的
             */
            while top.0 <= c.0 {
                //只要不是递减栈,一直弹出,并计算
                let mut yu = 0;
                let mut zhu = 0;
                yu = (i as i32 - top.1 as i32) * top.0;
                zhu = top.0 + c.3;
                yu -= zhu;
                c.2 = yu + top.2;
                c.3 = zhu + top.3;
                stack.pop();
                if stack.len() > 0 {
                    top = *stack.last().unwrap();
                } else {
                    break;
                }
            }
            println!("push={:?}", c);
            stack.push(c);
        }
        println!("stack={:?}", stack);
        /*
        栈上现在保存的由高到底的柱子,这时候要计算他们之间可以保存的雨水有多少
        */
        while stack.len() >= 2 {
            let top = stack.pop().unwrap();
            let last = *stack.last_mut().unwrap();
            let mut yu = top.0 * (top.1 as i32 - last.1 as i32);
            let zhu = top.0 + top.3;
            yu -= zhu;
            total += yu;
        }
        if stack.len() == 1 {
            total += stack[0].2;
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
