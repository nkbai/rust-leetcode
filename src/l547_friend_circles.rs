#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
547. 朋友圈
班上有 N 名学生。其中有些人是朋友，有些则不是。他们的友谊具有是传递性。如果已知 A 是 B 的朋友，B 是 C 的朋友，那么我们可以认为 A 也是 C 的朋友。所谓的朋友圈，是指所有朋友的集合。

给定一个 N * N 的矩阵 M，表示班级中学生之间的朋友关系。如果M[i][j] = 1，表示已知第 i 个和 j 个学生互为朋友关系，否则为不知道。你必须输出所有学生中的已知的朋友圈总数。

示例 1:

输入:
[[1,1,0],
 [1,1,0],
 [0,0,1]]
输出: 2
说明：已知学生0和学生1互为朋友，他们在一个朋友圈。
第2个学生自己在一个朋友圈。所以返回2。
示例 2:

输入:
[[1,1,0],
 [1,1,1],
 [0,1,1]]
输出: 1
说明：已知学生0和学生1互为朋友，学生1和学生2互为朋友，所以学生0和学生2也是朋友，所以他们三个在一个朋友圈，返回1。
注意：

N 在[1,200]的范围内。
对于所有学生，有M[i][i] = 1。
如果有M[i][j] = 1，则有M[j][i] = 1。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/friend-circles
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

 */

/*
并查集:
如果在i,j进行merge的时候,是true,则表明发现了新的集合.
朋友圈的数量减一
*/
struct Solution {}
struct DSU {
    pre: Vec<usize>,
}
impl DSU {
    pub fn find(&mut self, x: usize) -> usize {
        if self.pre[x] == x {
            return x;
        }
        let prex = self.find(self.pre[x]);
        self.pre[x] = prex;
        return prex;
    }
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let prex = self.find(x);
        let prey = self.find(y);
        if prex == prey {
            return false;
        }
        self.pre[prey] = prex;
        return true;
    }
}
impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut pre = vec![0; m.len()];
        for i in 0..m.len() {
            pre[i] = i;
        }
        let mut dsu = DSU { pre };
        let mut count = m.len();
        for i in 0..m.len() {
            for j in i + 1..m.len() {
                if m[i][j] == 1 {
                    if dsu.merge(i, j) {
                        count -= 1;
                    }
                }
            }
        }
        count as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]);
        assert_eq!(t, 2);
        let t = Solution::find_circle_num(vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
        ]);
        assert_eq!(t, 1);
    }
}
