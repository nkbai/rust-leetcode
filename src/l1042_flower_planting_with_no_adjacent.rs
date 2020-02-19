/*
有 N 个花园，按从 1 到 N 标记。在每个花园中，你打算种下四种花之一。

paths[i] = [x, y] 描述了花园 x 到花园 y 的双向路径。

另外，没有花园有 3 条以上的路径可以进入或者离开。

你需要为每个花园选择一种花，使得通过路径相连的任何两个花园中的花的种类互不相同。

以数组形式返回选择的方案作为答案 answer，其中 answer[i] 为在第 (i+1) 个花园中种植的花的种类。花的种类用  1, 2, 3, 4 表示。保证存在答案。



示例 1：

输入：N = 3, paths = [[1,2],[2,3],[3,1]]
输出：[1,2,3]
示例 2：

输入：N = 4, paths = [[1,2],[3,4]]
输出：[1,2,1,2]
示例 3：

输入：N = 4, paths = [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]]
输出：[1,2,3,4]


提示：

1 <= N <= 10000
0 <= paths.size <= 20000
不存在花园有 4 条或者更多路径可以进入或离开。
保证存在答案。
*/

//看别人评论,这里有个坑,就是只有四种花
struct Solution {}
impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: Vec<Vec<i32>> = (0..n).map(|_| Vec::new()).collect();
        for p in paths {
            adj[(p[0] - 1) as usize].push(p[1] - 1);
        }
        let mut marked: Vec<i32> = (0..n).map(|_| 0).collect();
        for i in 0..n {
            if marked[i as usize] == 0 {
                Solution::dfs(0, &adj, &mut marked);
            }
        }
        return marked;
    }
    fn dfs(c: i32, adj: &Vec<Vec<i32>>, marked: &mut Vec<i32>) {
        Solution::mark(c, marked, adj);
        for n in &adj[c as usize] {
            Solution::dfs(*n, adj, marked);
        }
    }
    #[allow(dead_code)]
    fn is_marked(c: i32, marked: &Vec<i32>) -> bool {
        return marked[c as usize] == 0;
    }
    fn mark(c: i32, marked: &mut Vec<i32>, adj: &Vec<Vec<i32>>) {
        let mut used: [i32; 4] = [0, 0, 0, 0];

        for p in &adj[c as usize] {
            if marked[*p as usize] > 0 {
                used[*p as usize] = 1;
            }
        }
        let mut canmark = false;

        for i in 0..4 {
            if used[i as usize] == 0 {
                marked[c as usize] = i + 1;
                canmark = true;
            }
        }
        assert!(canmark);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_garden_no_adj() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::garden_no_adj(
                4,
                vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![4, 1],
                    vec![1, 3],
                    vec![2, 4]
                ]
            )
        );
        /*
                输入：N = 3, paths = [[1,2],[2,3],[3,1]]
        输出：[1,2,3]
        示例 2：

        输入：N = 4, paths = [[1,2],[3,4]]
        输出：[1,2,1,2]
        示例 3：*/
        assert_eq!(
            vec![1, 2, 3],
            Solution::garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]])
        );
        assert_eq!(
            vec![1, 2, 1, 2],
            Solution::garden_no_adj(4, vec![vec![1, 2], vec![3, 4]])
        );
    }
}
