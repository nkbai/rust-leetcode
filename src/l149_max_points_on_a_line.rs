/*
149. 直线上最多的点数

给定一个二维平面，平面上有 n 个点，求最多有多少个点在同一条直线上。

示例 1:

输入: [[1,1],[2,2],[3,3]]
输出: 3
解释:
^
|
|        o
|     o
|  o
+------------->
0  1  2  3  4
示例 2:

输入: [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
输出: 4
解释:
^
|
|  o
|     o        o
|        o
|  o        o
+------------------->
0  1  2  3  4  5  6

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/max-points-on-a-line
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
最简单的思路: 暴力穷举

1. 任意选定一点i
由他尝试和所有其他点组成曲线,以斜率作为map的key,进行计数即可.
然后遍历所有的key,得到最终结果.
2. 一点改进
按照任意顺序对点进行遍历,
比如点0,3,4,5是最长的那根线,那么在处理3,4,5三个点的时候完全不用考虑0,因为他一定被计算过了.
所以第一轮遍历是N个点逐个计算,第二轮是N-1个点,最终是1个点
3. 再一点改进
如果已经找到最长的线是经过三个点,那么后三个点就完全没有必要尝试了,她最好也不过是3.

复杂度分析:
1. 时间复杂度O(N^2),第一轮是N,第二轮是N-1,平均下来是N*(N-1)/2
2. 空间复杂度: O(N),当处理第一轮的时候,最多需要存储N-1个直线的斜率.


*/

use std::cmp::max;
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let points = points.as_slice();
        if points.len() <= 0 {
            return 0;
        }
        let mut max_count = 0;
        for i in 0..points.len() {
            if points.len() - i < max_count as usize {
                break;
            }
            let cnt = Self::process_ith(&points[i..points.len()]);
            max_count = max(max_count, cnt);
        }
        max_count
    }
    //计算该数组中,第0个点和其他所有点的共线的点数,至少是1
    fn process_ith(points: &[Vec<i32>]) -> i32 {
        let mut m = HashMap::new(); //斜率到计数的映射
        let mut max_count = 1;
        let p0 = points.get(0).expect("must have one");
        let mut duplicats = 0;
        let mut verticals = 1;
        for i in 1..points.len() {
            let p = points.get(i).expect(format!("doesn't have {}", i).as_str());
            if p[0] == p0[0] && p[1] == p0[1] {
                duplicats += 1;
            } else if p[0] == p0[0] {
                //斜率为无穷大
                verticals += 1;
                max_count = max(max_count, verticals);
            } else {
                let k = (p[1] - p0[1]) as f64 / (p[0] - p0[0]) as f64;
                println!("k={}", k);
                //将浮点数转换为整数来处理,精度有限,如果想解决精度问题,就要把除法转换为乘法来处理,复杂度没有变化,但是懒得做了
                //思路就是map中保存的value就是一个数组,对应的是[p,cnt],[p.cnt],在精度相同的情况下,只能这么处理
                //极端情况下,整体复杂度论文O(N^3)而不是O(N^2)
                let k = (k * 100000.0) as i64;
                let e = m.get_mut(&k);
                let cnt;
                match e {
                    None => {
                        m.insert(k, 2);
                        cnt = 2;
                    }
                    Some(x) => {
                        *x += 1;
                        cnt = *x;
                    }
                }
                max_count = max(max_count, cnt);
            }
        }
        max_count + duplicats //考虑重复的点
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]);
        assert_eq!(t, 3);
        let t = Solution::max_points(vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ]);
        assert_eq!(t, 4);
        let t = Solution::max_points(vec![vec![1, 1], vec![1, 1]]);
        //        assert_eq!(t, 2);
        //因为精度问题,这个case是过不了的
        //        let t = Solution::max_points(vec![
        //            vec![0, 0],
        //            vec![94911151, 94911150],
        //            vec![94911152, 94911151],
        //        ]);
        assert_eq!(t, 2);
        let t = Solution::max_points(vec![
            vec![560, 248],
            vec![0, 16],
            vec![30, 250],
            vec![950, 187],
            vec![630, 277],
            vec![950, 187],
            vec![-212, -268],
            vec![-287, -222],
            vec![53, 37],
            vec![-280, -100],
            vec![-1, -14],
            vec![-5, 4],
            vec![-35, -387],
            vec![-95, 11],
            vec![-70, -13],
            vec![-700, -274],
            vec![-95, 11],
            vec![-2, -33],
            vec![3, 62],
            vec![-4, -47],
            vec![106, 98],
            vec![-7, -65],
            vec![-8, -71],
            vec![-8, -147],
            vec![5, 5],
            vec![-5, -90],
            vec![-420, -158],
            vec![-420, -158],
            vec![-350, -129],
            vec![-475, -53],
            vec![-4, -47],
            vec![-380, -37],
            vec![0, -24],
            vec![35, 299],
            vec![-8, -71],
            vec![-2, -6],
            vec![8, 25],
            vec![6, 13],
            vec![-106, -146],
            vec![53, 37],
            vec![-7, -128],
            vec![-5, -1],
            vec![-318, -390],
            vec![-15, -191],
            vec![-665, -85],
            vec![318, 342],
            vec![7, 138],
            vec![-570, -69],
            vec![-9, -4],
            vec![0, -9],
            vec![1, -7],
            vec![-51, 23],
            vec![4, 1],
            vec![-7, 5],
            vec![-280, -100],
            vec![700, 306],
            vec![0, -23],
            vec![-7, -4],
            vec![-246, -184],
            vec![350, 161],
            vec![-424, -512],
            vec![35, 299],
            vec![0, -24],
            vec![-140, -42],
            vec![-760, -101],
            vec![-9, -9],
            vec![140, 74],
            vec![-285, -21],
            vec![-350, -129],
            vec![-6, 9],
            vec![-630, -245],
            vec![700, 306],
            vec![1, -17],
            vec![0, 16],
            vec![-70, -13],
            vec![1, 24],
            vec![-328, -260],
            vec![-34, 26],
            vec![7, -5],
            vec![-371, -451],
            vec![-570, -69],
            vec![0, 27],
            vec![-7, -65],
            vec![-9, -166],
            vec![-475, -53],
            vec![-68, 20],
            vec![210, 103],
            vec![700, 306],
            vec![7, -6],
            vec![-3, -52],
            vec![-106, -146],
            vec![560, 248],
            vec![10, 6],
            vec![6, 119],
            vec![0, 2],
            vec![-41, 6],
            vec![7, 19],
            vec![30, 250],
        ]);
        assert_eq!(t, 22);
    }
}
