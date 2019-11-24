/*
463. 岛屿的周长


给定一个包含 0 和 1 的二维网格地图，其中 1 表示陆地 0 表示水域。

网格中的格子水平和垂直方向相连（对角线方向不相连）。整个网格被水完全包围，但其中恰好有一个岛屿（或者说，一个或多个表示陆地的格子相连组成的岛屿）。

岛屿中没有“湖”（“湖” 指水域在岛屿内部且不和岛屿周围的水相连）。格子是边长为 1 的正方形。网格为长方形，且宽度和高度均不超过 100 。计算这个岛屿的周长。



示例 :

输入:
[[0,1,0,0],
 [1,1,1,0],
 [0,1,0,0],
 [1,1,0,0]]

输出: 16

解释: 它的周长是下面图片中的 16 个黄色的边：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/island.png)

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/island-perimeter
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
遍历一遍
1. 统计1的个数
2. 统计1向右向下邻居是1的个数
结果就是1的个数*4-2中个数*2
*/

struct Solution {}
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        if r == 0 {
            return 0;
        }
        let c = grid[0].len();
        if c == 0 {
            return 0;
        }

        let mut cnt1 = 0;
        let mut cnt_neighbor = 0;
        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == 1 {
                    cnt1 += 1;
                } else {
                    continue;
                }
                if i + 1 < r && grid[i + 1][j] == 1 {
                    cnt_neighbor += 1;
                    //                    println!("i={},j={},has heighbor", i, j);
                }
                if j + 1 < c && grid[i][j + 1] == 1 {
                    cnt_neighbor += 1;
                    //                    println!("i={},j={},has heighbor2", i, j);
                }
            }
        }
        //        println!("cnt1={},neighbor={}", cnt1, cnt_neighbor);
        (cnt1 * 4 - cnt_neighbor * 2) as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
    }
}
