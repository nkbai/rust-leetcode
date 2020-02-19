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
*/
use std::collections::{BTreeMap, HashMap, HashSet};

struct Solution {}
struct Node {
    val: i32,
    row: usize,
    col: usize,
}
impl Node {
    fn new(val: i32, row: usize, col: usize) -> Self {
        Node { val, row, col }
    }
}
impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut m = BTreeMap::new();
        //建立一张图
        for i in 0..forest.len() {
            for j in 0..forest[0].len() {
                if forest[i][j] > 1 {
                    let n = Node::new(forest[i][j], i, j);
                    m.insert(n.val, n);
                }
            }
        }
        if forest[0][0] == 0 {
            return -1; //第一步无法走
        }

        let mut start = (0, 0);
        let mut mark = HashSet::new();
        let mut mins = HashMap::new();
        let total_row = forest.len();
        let total_col = forest[0].len();
        let step = Self::min_step(
            start,
            (total_row - 1, total_col - 1),
            &forest,
            &mut mark,
            &mut mins,
        );
        if step >= 0 && forest[total_row - 1][total_col - 1] > 0 {
            mins.insert((start.0, start.1, total_row - 1, total_col - 1), step);
        }
        let mut total_step = 0;
        for (_, n) in m {
            let step = mins.get(&(start.0, start.1, n.row, n.col));
            if step.is_none() {
                return -1;
            }
            let step = *step.unwrap();
            start = (n.row, n.col);
            total_step += step;
        }
        total_step
    }
    //求从start到target所有可能的路径
    fn min_step(
        start: (usize, usize),
        target: (usize, usize),
        forest: &Vec<Vec<i32>>,
        mark: &mut HashSet<(usize, usize)>,
        mins: &mut HashMap<(usize, usize, usize, usize), i32>,
    ) -> i32 {
        if start == target {
            return 0;
        }

        let mut step = std::i32::MAX;
        let total_row = forest.len();
        let total_col = forest[0].len();
        let next_steps = Self::get_next(start, total_row, total_col);
        //分别尝试前后左右
        next_steps.iter().for_each(|next_step| {
            if !mark.contains(next_step) && forest[next_step.0][next_step.1] > 0 {
                mark.insert(*next_step);
                let curent_step = Self::min_step(*next_step, target, forest, mark, mins);
                if curent_step >= 0 && curent_step + 1 < step {
                    let s = mins.get_mut(&(start.0, start.1, next_step.0, next_step.1));
                    match s {
                        Some(s) => {
                            if *s > curent_step {
                                *s = curent_step;
                            }
                        }
                        None => {
                            mins.insert((start.0, start.1, next_step.0, next_step.1), curent_step);
                        }
                    }

                    step = curent_step + 1;
                }
                mark.remove(next_step);
            }
        });
        if step != std::i32::MAX {
            return step;
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
