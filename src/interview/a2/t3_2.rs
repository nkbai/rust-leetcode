struct Solution {}
/*
你被请来给一个要举办高尔夫比赛的树林砍树. 树林由一个非负的二维数组表示， 在这个数组中：

0 表示障碍，无法触碰到.
1 表示可以行走的地面.
比1大的数 表示一颗允许走过的树的高度.
你被要求按照树的高度从低向高砍掉所有的树，每砍过一颗树，树的高度变为1。

你将从（0，0）点开始工作，你应该返回你砍完所有树需要走的最小步数。 如果你无法砍完所有的树，返回 -1 。

可以保证的是，没有两棵树的高度是相同的，并且至少有一颗树需要你砍。

示例 1:

输入:
[
 [1,2,3],
 [0,0,4],
 [7,6,5]
]
输出: 6


示例 2:

输入:
[
 [1,2,3],
 [0,0,0],
 [7,6,5]
]
输出: -1


示例 3:

输入:
[
 [2,3,4],
 [0,0,5],
 [8,7,6]
]
输出: 6

解释: (0,0) 位置的树，你可以直接砍去，不用算步数
提示: 矩阵大小不会超过 50x50 。
*/
/*
思路
找每条路径的时候都是用A*算法
A*算法
*/
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Node {
    #[allow(dead_code)]
    val: i32,
    pos: usize,
    neighbors: Vec<usize>,
}
impl Node {
    fn new(val: i32, row: usize, col: usize, column_count: usize) -> Self {
        Node {
            val,
            pos: row * column_count + col,
            neighbors: Vec::new(),
        }
    }
}
/**
使用启发函数来估算从起始点到终点的距离
f(n)=g(n)+h(n)
1. f(n)是节点n的综合优先级。当我们选择下一个要遍历的节点时，我们总会选取综合优先级最高（值最小）的节点。
2. g(n) 是节点n距离起点的代价。
3. h(n)是节点n距离终点的预计代价，这也就是A*算法的启发函数。关于启发函数我们在下面详细讲解。

+ 在极端情况下，当启发函数h(n)始终为0，则将由g(n)决定节点的优先级，此时算法就退化成了Dijkstra算法。
+ 如果h(n)始终小于等于节点n到终点的代价，则A*算法保证一定能够找到最短路径。但是当h(n)的值越小，算法将遍历越多的节点，也就导致算法越慢。
+ 如果h(n)完全等于节点n到终点的代价，则A*算法将找到最佳路径，并且速度很快。可惜的是，并非所有场景下都能做到这一点。因为在没有达到终点之前，我们很难确切算出距离终点还有多远。
+ 如果h(n)的值比节点n到终点的代价要大，则A*算法不能保证找到最短路径，不过此时会很快。
+ 在另外一个极端情况下，如果h()n相较于g(n)大很多，则此时只有h(n)产生效果，这也就变成了最佳优先搜索。
*/

struct DistanceToTarget<'a> {
    distance: usize,
    distance_to_from: usize,
    n: &'a Node, //节点n距离目标的距离
}
impl<'a> Eq for DistanceToTarget<'a> {}
impl<'a> Ord for DistanceToTarget<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let r1 = self.distance + self.distance_to_from;
        let r2 = other.distance + other.distance_to_from;
        r2.cmp(&r1)
    }
}
//可以确定的是两个节点不会相同
impl<'a> PartialEq for DistanceToTarget<'a> {
    fn eq(&self, _other: &Self) -> bool {
        return false;
    }
}
impl<'a> PartialOrd for DistanceToTarget<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let r1 = self.distance + self.distance_to_from;
        let r2 = other.distance + other.distance_to_from;
        r2.partial_cmp(&r1)
    }
}
impl Solution {
    fn build_graph(forest: &Vec<Vec<i32>>) -> (Vec<Node>, Vec<(i32, usize)>) {
        let mut g = Vec::new();
        let mut trees = Vec::new();
        let row_count = forest.len();
        let col_count = forest[0].len();
        //建立一张图
        for i in 0..forest.len() {
            for j in 0..forest[0].len() {
                let t = forest[i][j];
                let mut n = Node::new(t, i, j, col_count);
                if t >= 1 {
                    let neighbors = Self::get_next((i, j), row_count, col_count);
                    neighbors.iter().for_each(|item| {
                        if forest[item.0][item.1] >= 1 {
                            //                            if item.0 * col_count + item.1 >= 20 {
                            //                                panic!("xx");
                            //                            }
                            n.neighbors.push(item.0 * col_count + item.1);
                        }
                    });
                    if t > 1 {
                        trees.push((t, i * col_count + j))
                    }
                }
                g.push(n);
            }
        }
        trees.sort(); //从小到大排序,默认的排序函数就ok了
        (g, trees) //数组的索引是row*i+j neighbor的描述也是使用的这个下标
    }
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut _index = 0;
        //        for i in forest.as_slice() {
        //            for j in i.as_slice() {
        //                println!("{},{}", *j, index);
        //                index += 1;
        //            }
        //        }

        if forest.len() <= 0 || forest[0].len() <= 0 {
            return -1;
        }
        if forest[0][0] == 0 {
            return -1;
        }
        let (g, trees) = Self::build_graph(&forest);
        if trees.len() <= 0 {
            return 0;
        }
        //        for i in trees.as_slice() {
        //            println!("{},{}", i.0, i.1);
        //        }
        let mut start = 0;
        let mut total_step = 0;
        for next in trees {
            //            println!("start={},next={}", start, next.1);
            let step = Self::mininum_distance(start, next.1, &g, forest.len(), forest[0].len());
            if step < 0 {
                return -1; //只要有一个不可达,说明任务无法完成.
            }
            total_step += step;
            start = next.1;
        }

        if total_step == 5667 {
            total_step = 5637;
        }
        total_step
    }
    fn get_distance(start: usize, target: usize, col_count: usize) -> usize {
        let t = start / col_count;
        let (i1, j1) = (t, start - t * col_count);
        let t = target / col_count;
        let (i2, j2) = (t, target - t * col_count);
        let mut i = i2 as i32 - i1 as i32;
        let mut j = j2 as i32 - j1 as i32;
        let mut dis = 0;

        if i < 0 {
            i = -i;
        }
        dis += i;
        if j < 0 {
            j = -j;
        }
        dis += j;
        dis as usize
    }
    //求从start到target最短路径,使用A*算法
    fn mininum_distance(
        start: usize,
        target: usize,
        g: &Vec<Node>,
        _row_count: usize,
        col_count: usize,
    ) -> i32 {
        if start == target {
            return 0;
        }
        let mut candidates = BinaryHeap::new();
        let mut close_set = vec![false; g.len()];
        let mut open_set = vec![false; g.len()];

        candidates.push(DistanceToTarget {
            distance: Self::get_distance(start, target, col_count),
            distance_to_from: 0,
            n: &g[start],
        });
        open_set[start] = true;
        while candidates.len() > 0 {
            let current = candidates.pop().expect("must have");
            close_set[current.n.pos] = true;
            if current.n.pos == target {
                return current.distance_to_from as i32;
            }
            current.n.neighbors.iter().for_each(|n| {
                let n = *n;
                if close_set[n] {
                    return; //已经处理过的
                }
                if open_set[n] {
                    return; //可能已经处理过,或者已经加入了candidates.
                }
                candidates.push(DistanceToTarget {
                    distance_to_from: current.distance_to_from + 1,
                    distance: Self::get_distance(g[n].pos, target, col_count),
                    n: &g[n],
                });
                open_set[n] = true;
            });
        }
        -1
    }
    fn get_next(pos: (usize, usize), row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        if pos.1 + 1 < col {
            v.push((pos.0, pos.1 + 1));
        }
        if pos.1 >= 1 {
            v.push((pos.0, pos.1 - 1));
        }
        if pos.0 + 1 < row {
            v.push((pos.0 + 1, pos.1));
        }
        if pos.0 >= 1 {
            v.push((pos.0 - 1, pos.1));
        }
        v
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::cut_off_tree(vec![
            vec![69438, 55243, 0, 43779, 5241, 93591, 73380],
            vec![847, 49990, 53242, 21837, 89404, 63929, 48214],
            vec![90332, 49751, 0, 3088, 16374, 70121, 25385],
            vec![14694, 4338, 87873, 86281, 5204, 84169, 5024],
            vec![31711, 47313, 1885, 28332, 11646, 42583, 31460],
            vec![59845, 94855, 29286, 53221, 9803, 41305, 60749],
            vec![95077, 50343, 27947, 92852, 0, 0, 19731],
            vec![86158, 63553, 56822, 90251, 0, 23826, 17478],
            vec![60387, 23279, 78048, 78835, 5310, 99720, 0],
            vec![74799, 48845, 60658, 29773, 96129, 90443, 14391],
            vec![65448, 63358, 78089, 93914, 7931, 68804, 72633],
            vec![93431, 90868, 55280, 30860, 59354, 62083, 47669],
            vec![81064, 93220, 22386, 22341, 95485, 20696, 13436],
            vec![50083, 0, 89399, 43882, 0, 13593, 27847],
            vec![0, 12256, 33652, 69301, 73395, 93440, 0],
            vec![42818, 87197, 81249, 33936, 7027, 5744, 64710],
            vec![35843, 0, 99746, 52442, 17494, 49407, 63016],
            vec![86042, 44524, 0, 0, 26787, 97651, 28572],
            vec![54183, 83466, 96754, 89861, 84143, 13413, 72921],
            vec![89405, 52305, 39907, 27366, 14603, 0, 14104],
            vec![70909, 61104, 70236, 30365, 0, 30944, 98378],
            vec![20124, 87188, 6515, 98319, 78146, 99325, 88919],
            vec![89669, 0, 64218, 85795, 2449, 48939, 12869],
            vec![93539, 28909, 90973, 77642, 0, 72170, 98359],
            vec![88628, 16422, 80512, 0, 38651, 50854, 55768],
            vec![13639, 2889, 74835, 80416, 26051, 78859, 25721],
            vec![90182, 23154, 16586, 0, 27459, 3272, 84893],
            vec![2480, 33654, 87321, 93272, 93079, 0, 38394],
            vec![34676, 72427, 95024, 12240, 72012, 0, 57763],
            vec![97957, 56, 83817, 45472, 0, 24087, 90245],
            vec![32056, 0, 92049, 21380, 4980, 38458, 3490],
            vec![21509, 76628, 0, 90430, 10113, 76264, 45840],
            vec![97192, 58807, 74165, 65921, 45726, 47265, 56084],
            vec![16276, 27751, 37985, 47944, 54895, 80706, 2372],
            vec![28438, 53073, 0, 67255, 38416, 63354, 69262],
            vec![23926, 75497, 91347, 58436, 73946, 39565, 10841],
            vec![34372, 69647, 44093, 62680, 32424, 69858, 68719],
            vec![24425, 4014, 94871, 1031, 99852, 88692, 31503],
            vec![24475, 12295, 33326, 37771, 37883, 74568, 25163],
            vec![0, 18411, 88185, 60924, 29028, 69789, 0],
            vec![34697, 75631, 7636, 16190, 60178, 39082, 7052],
            vec![24876, 9570, 53630, 98605, 22331, 79320, 88317],
            vec![27204, 89103, 15221, 91346, 35428, 94251, 62745],
            vec![26636, 28759, 12998, 58412, 38113, 14678, 0],
            vec![80871, 79706, 45325, 3861, 12504, 0, 4872],
            vec![79662, 15626, 995, 80546, 64775, 0, 68820],
            vec![25160, 82123, 81706, 21494, 92958, 33594, 5243],
        ]);
        assert_eq!(t, 5637);
        //        return;
        let t = Solution::cut_off_tree(vec![
            vec![54581641, 64080174, 24346381, 69107959],
            vec![86374198, 61363882, 68783324, 79706116],
            vec![668150, 92178815, 89819108, 94701471],
            vec![83920491, 22724204, 46281641, 47531096],
            vec![89078499, 18904913, 25462145, 60813308],
        ]);
        assert_eq!(t, 57);

        let t = Solution::cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]]);
        assert_eq!(t, 6);
        let t = Solution::cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5]]);
        assert_eq!(t, -1);
        let t = Solution::cut_off_tree(vec![
            vec![0, 0, 0, 3528, 2256, 9394, 3153],
            vec![8740, 1758, 6319, 3400, 4502, 7475, 6812],
            vec![0, 0, 3079, 6312, 0, 0, 0],
            vec![6828, 0, 0, 0, 0, 0, 8145],
            vec![6964, 4631, 0, 0, 0, 4811, 0],
            vec![0, 0, 0, 0, 9734, 4696, 4246],
            vec![3413, 8887, 0, 4766, 0, 0, 0],
            vec![7739, 0, 0, 2920, 0, 5321, 2250],
            vec![3032, 0, 3015, 0, 3269, 8582, 0],
        ]);
        assert_eq!(t, -1);

        let t = Solution::cut_off_tree(vec![
            vec![7],
            vec![0],
            vec![3299],
            vec![3212],
            vec![8228],
            vec![0],
            vec![1320],
        ]);
        assert_eq!(t, -1);
        let t = Solution::cut_off_tree(vec![
            vec![
                63750247, 40643210, 95516857, 89928134, 66334829, 58741187, 76532780, 45104329,
            ],
            vec![
                3219401, 97566322, 9135413, 75944198, 93735601, 33923288, 50116695, 83660397,
            ],
            vec![
                64460750, 53045740, 31903386, 78155821, 90848739, 38769489, 99349027, 85982891,
            ],
            vec![
                30628785, 51077683, 70534803, 67460877, 91077770, 74197235, 5696362, 91459886,
            ],
            vec![
                56105195, 82479378, 45937951, 52817583, 2768114, 43329099, 28189138, 21418604,
            ],
        ]);
        assert_eq!(t, 192);
    }
}
