use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;
struct Edge {
    node: usize,
    cost: usize,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cmp(&self)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    //默认是最大堆
    let mut h = BinaryHeap::new();
    let mut dist: Vec<usize> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    dist[start] = 0;
    h.push(State {
        node: start,
        cost: 0,
    });
    while let Some(State {
        node: pos,
        cost: cost,
    }) = h.pop()
    {
        println!("process node={},cost={}", pos, cost);
        //找到了一个更小的路径,如果是目标就结束
        if pos == goal {
            return Some(cost);
        }
        //比现在已知的还大
        if cost > dist[pos] {
            continue;
        }
        //如果不是目标,沿着他扩展
        for edge in &adj_list[pos] {
            let next = State {
                node: edge.node,
                cost: dist[pos] + edge.cost,
            };
            //对于那些比现有路径还长的,直接忽略,找到了新的路径,才扩展到堆中
            if next.cost < dist[edge.node] {
                dist[edge.node] = next.cost;
                h.push(next);
                println!("heap={:?}", h)
            }
        }
    }
    return None;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testshortest() {
        let graph = vec![
            //0
            vec![Edge { node: 1, cost: 1 }, Edge { node: 2, cost: 10 }],
            //1
            vec![Edge { node: 3, cost: 2 }],
            //2
            vec![
                Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 },
            ],
            //3
            vec![Edge { node: 4, cost: 2 }, Edge { node: 0, cost: 7 }],
            vec![],
        ];
        let n = shortest_path(&graph, 0, 4);
        assert_eq!(n, Some(5));
        let n = shortest_path(&graph, 0, 3);
        assert_eq!(n, Some(3));
    }
}
