#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
210. 课程表 II
现在你总共有 n 门课需要选，记为 0 到 n-1。

在选修某些课程之前需要一些先修课程。 例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示他们: [0,1]

给定课程总量以及它们的先决条件，返回你为了学完所有课程所安排的学习顺序。

可能会有多个正确的顺序，你只要返回一种就可以了。如果不可能完成所有课程，返回一个空数组。

示例 1:

输入: 2, [[1,0]]
输出: [0,1]
解释: 总共有 2 门课程。要学习课程 1，你需要先完成课程 0。因此，正确的课程顺序为 [0,1] 。
示例 2:

输入: 4, [[1,0],[2,0],[3,1],[3,2]]
输出: [0,1,2,3] or [0,2,1,3]
解释: 总共有 4 门课程。要学习课程 3，你应该先完成课程 1 和课程 2。并且课程 1 和课程 2 都应该排在课程 0 之后。
因此，一个正确的课程顺序是 [0,1,2,3] 。另一个正确的排序是 [0,2,1,3] 。
说明:

输入的先决条件是由边缘列表表示的图形，而不是邻接矩阵。详情请参见图的表示法。
你可以假定输入的先决条件中没有重复的边。
提示:

这个问题相当于查找一个循环是否存在于有向图中。如果存在循环，则不存在拓扑排序，因此不可能选取所有课程进行学习。
通过 DFS 进行拓扑排序 - 一个关于Coursera的精彩视频教程（21分钟），介绍拓扑排序的基本概念。
拓扑排序也可以通过 BFS 完成。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/course-schedule-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
和207是一样的,只不过要返回收集的结果,而这个我们已经做了.

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
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
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
        if r.len() == num_courses as usize {
            r
        } else {
            vec![]
        }
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
        let t = Solution::find_order(2, vec![vec![1, 0]]);
        assert_eq!(t, vec![0, 1]);

        let t = Solution::find_order(2, vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(t, vec![]);
    }
}
