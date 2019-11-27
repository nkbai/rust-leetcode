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
递增栈思路
栈中保存的是数组的下标,但是要求a[i]<a[j]
stack=>| 0 1 2 ,5,7 | 要求a[0]<a[1]<a[2]<a[5]<a[7] 注意是严格递增,不能是相等
一开始压栈-1,
假设这时候站内是|-1,0,1,2,7| 高度分别是|0,2,7,8,9|,那么这时候来a[8]高度为5,则必须一直弹出直到a[0]=2留下
|0,8| 对应的高度分别是|2,5|
但是在弹出的过程中要一个一个计算面积
比如弹出9的时候,得到面积是9x(8-2-1) //可以确定的是从下标2到7被弹出去的那部分肯定都是大于9的,否则它们就会被保留下来.
弹出8的时候得到面积是8x(8-1-1) //可以确定的是从下标1到8弹出去的值都是大于8的,否则它们就会被保留下来.
计算面积公式是
a[stack[top]]*(i-stack[top-1]-1)
当全部压栈后,顺着同样的思路出栈,
只是注意一点这时候的计算公式就是
a[stack[top]]*(stack[top]-stack[top-1])
*/

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
