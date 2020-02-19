/*
554. 砖墙

你的面前有一堵方形的、由多行砖块组成的砖墙。 这些砖块高度相同但是宽度不同。你现在要画一条自顶向下的、穿过最少砖块的垂线。

砖墙由行的列表表示。 每一行都是一个代表从左至右每块砖的宽度的整数列表。

如果你画的线只是从砖块的边缘经过，就不算穿过这块砖。你需要找出怎样画才能使这条线穿过的砖块数量最少，并且返回穿过的砖块数量。

你不能沿着墙的两个垂直边缘之一画线，这样显然是没有穿过一块砖的。



示例：

输入: [[1,2,2,1],
      [3,1,2],
      [1,3,2],
      [2,4],
      [3,1,2],
      [1,3,1,1]]

输出: 2

解释:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/brick_wall.png)


提示：

每一行砖块的宽度之和应该相等，并且不能超过 INT_MAX。
每一行砖块的数量在 [1,10,000] 范围内， 墙的高度在 [1,10,000] 范围内， 总的砖块数量不超过 20,000。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/brick-wall
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
1. 每一行的总和都是固定的,比如这里是6
2. 遍历每一行,统计边缘,比如第一行的边缘数是1,3,5 0和6肯定不能考虑进去
3. 统计哪个数对应的边缘数最大,那么经过的最少砖数就是高度减去这个边缘数.

如果一条线经过了最多的边缘,那么他就是要找那根线,
复杂度分析:
砖块数位N,则时间复杂度位O(N),空间复杂对最糟糕的情况下也是O(N),就是任意两个砖块都不对齐的情况下
*/

use std::cmp::max;
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut m = HashMap::new();
        let mut sum;
        let mut max_count = 0;
        let row_count = wall.len() as i32;
        for row in wall {
            sum = 0;
            //不能计算最后一列
            for col in 0..row.len() - 1 {
                sum += row[col];
                let cnt;
                let v = m.get_mut(&sum);
                match v {
                    None => {
                        m.insert(sum, 1);
                        cnt = 1;
                    }
                    Some(v) => {
                        *v += 1;
                        cnt = *v;
                    }
                }
                max_count = max(max_count, cnt);
            }
        }
        println!("row_count={},max={}", row_count, max_count);
        row_count - max_count
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        /*
          [[1,2,2,1],
        [3,1,2],
        [1,3,2],
        [2,4],
        [3,1,2],
        [1,3,1,1]]
        */
        let t = Solution::least_bricks(vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1],
        ]);
        assert_eq!(t, 2);
    }
}
