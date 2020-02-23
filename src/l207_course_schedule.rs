#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
207. 课程表
现在你总共有 n 门课需要选，记为 0 到 n-1。

在选修某些课程之前需要一些先修课程。 例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示他们: [0,1]

给定课程总量以及它们的先决条件，判断是否可能完成所有课程的学习？

示例 1:

输入: 2, [[1,0]]
输出: true
解释: 总共有 2 门课程。学习课程 1 之前，你需要完成课程 0。所以这是可能的。
示例 2:

输入: 2, [[1,0],[0,1]]
输出: false
解释: 总共有 2 门课程。学习课程 1 之前，你需要先完成​课程 0；并且学习课程 0 之前，你还应先完成课程 1。这是不可能的。
说明:

输入的先决条件是由边缘列表表示的图形，而不是邻接矩阵。详情请参见图的表示法。
你可以假定输入的先决条件中没有重复的边。
提示:

这个问题相当于查找一个循环是否存在于有向图中。如果存在循环，则不存在拓扑排序，因此不可能选取所有课程进行学习。
通过 DFS 进行拓扑排序 - 一个关于Coursera的精彩视频教程（21分钟），介绍拓扑排序的基本概念。
拓扑排序也可以通过 BFS 完成。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/course-schedule
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

 */

/*
思路:
首先,用不着Set之类的高级工具,因为课程的编号恰好就是数组的下标.
广度优先遍历,从入度为0的节点开始,指导所有的节点入度都为0,
否则就认为不可能.
*/
struct Solution {}
#[derive(Default, Clone)]
struct Node {
    in_degrees: i32,
    neighbors: Vec<i32>,
}
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut r = Vec::new();
        let mut g = Self::build_graph(num_courses, prerequisites);
        let mut stack = Self::found_in_dregress0(&g);
        while let Some(n) = stack.pop() {
            r.push(n);
            /*
            避免clone的方法是把in_degrees和neighbors分开
            不要放在同一个结构中
            */
            let n = g[n as usize].clone();
            for p in n.neighbors.iter() {
                let node = &mut g[*p as usize];
                node.in_degrees -= 1;
                if node.in_degrees == 0 {
                    stack.push(*p);
                }
            }
        }
        r.len() == num_courses as usize
    }
    fn build_graph(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<Node> {
        let mut r = vec![Node::default(); num_courses as usize];
        for p in prerequisites.iter() {
            let from = &mut r[p[1] as usize];
            from.neighbors.push(p[0]);
            let to = &mut r[p[0] as usize];
            to.in_degrees += 1;
        }
        r
    }
    fn found_in_dregress0(v: &Vec<Node>) -> Vec<i32> {
        let mut r = Vec::new();
        for (pos, n) in v.iter().enumerate() {
            if n.in_degrees == 0 {
                r.push(pos as i32);
            }
        }
        r
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::can_finish(2, vec![vec![1, 0]]);
        assert_eq!(t, true);

        let t = Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(t, false);
    }
}
