/*
488. 祖玛游戏
回忆一下祖玛游戏。现在桌上有一串球，颜色有红色(R)，黄色(Y)，蓝色(B)，绿色(G)，还有白色(W)。 现在你手里也有几个球。

每一次，你可以从手里的球选一个，然后把这个球插入到一串球中的某个位置上（包括最左端，最右端）。接着，如果有出现三个或者三个以上颜色相同的球相连的话，就把它们移除掉。重复这一步骤直到桌上所有的球都被移除。

找到插入并可以移除掉桌上所有球所需的最少的球数。如果不能移除桌上所有的球，输出 -1 。

示例:
输入: "WRRBBW", "RB"
输出: -1
解释: WRRBBW -> WRR[R]BBW -> WBBW -> WBB[B]W -> WW （翻译者标注：手上球已经用完，桌上还剩两个球无法消除，返回-1）

输入: "WWRRBBWW", "WRBRW"
输出: 2
解释: WWRRBBWW -> WWRR[R]BBWW -> WWBBWW -> WWBB[B]WW -> WWWW -> empty

输入:"G", "GGGGG"
输出: 2
解释: G -> G[G] -> GG[G] -> empty

输入: "RBYYBBRRB", "YRBGB"
输出: 3
解释: RBYYBBRRB -> RBYY[Y]BBRRB -> RBBBRRB -> RRRB -> B -> B[B] -> BB[B] -> empty
标注:

你可以假设桌上一开始的球中，不会有三个及三个以上颜色相同且连着的球。
桌上的球不会超过20个，输入的数据中代表这些球的字符串的名字是 "board" 。
你手中的球不会超过5个，输入的数据中代表这些球的字符串的名字是 "hand"。
输入的两个字符串均为非空字符串，且只包含字符 'R','Y','B','G','W'。
*/
/*
思路:
 暴力穷举
 因为可以举例证明贪心不可靠
 比如先找两个连着的,消除后再去尝试其他
 这种情况就是错的
 WBWW 这种情况放两个B就是最优选择
*/
struct Solution {}
impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        //RYBGW 只有这五种情况,分别映射为0,1,2,3,4 简单起见
        let board = Self::map_board(board);
        let hand = Self::map_hand(hand);
        //        println!("board={:?}", board);
        //        println!("hand={:?}", hand);
        let mut min_step = std::i32::MAX;
        for i in 0..board.len() {
            let c = Self::try_step(i, board.clone(), hand.clone(), 0);
            if c > 0 && min_step > c {
                min_step = c;
            }
            //            println!("try c={},i={},min_step={}", c, i, min_step);
        }
        if min_step == std::i32::MAX {
            min_step = -1;
        }
        min_step
    }
    fn try_step(start: usize, mut board: Vec<(i32, i32)>, mut hand: [i32; 5], current: i32) -> i32 {
        //        println!("try start={}, current={}", start, current);
        //        println!("board={:?},hand={:?}", board, hand);

        let pick = board[start];
        let need = 3 - pick.1;
        if hand[pick.0 as usize] < need {
            return -1;
        }
        //可以移除一个了,
        Self::remove_and_merge(&mut board, start);
        hand[pick.0 as usize] -= need;
        if board.len() == 0 {
            println!("found={}", current + need);
            return current + need; //找到了一种方法,
        }
        let mut current_min_step = std::i32::MAX;
        for i in 0..board.len() {
            let c = Self::try_step(i, board.clone(), hand.clone(), current + need);
            if c > 0 && current_min_step > c {
                current_min_step = c;
            }
        }
        if current_min_step != std::i32::MAX {
            return current_min_step;
        }
        -1
    }
    fn remove_and_merge(board: &mut Vec<(i32, i32)>, i: usize) {
        board.remove(i);
        if board.len() > i && i >= 1 {
            //移除的是中间的某个数值,如果移除的是0,或者最后一个,肯定不会合并
            let mut left = board[i - 1];
            let right = board[i];
            if left.0 == right.0 {
                left.1 += right.1;
                board.remove(i);
                if left.1 >= 3 {
                    Self::remove_and_merge(board, i - 1);
                } else {
                    board[i - 1] = left;
                }
            }
        }
    }
    //每个字符以及其出现的次数
    fn map_board(board: String) -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        let mut last = -1;
        let mut last_count = -1;
        for i in board.as_bytes() {
            let c = match *i as char {
                'R' => 0,
                'Y' => 1,
                'B' => 2,
                'G' => 3,
                'W' => 4,
                _ => panic!("not possible"),
            };
            if c != last {
                if last_count > 0 {
                    v.push((last, last_count));
                }
                last = c;
                last_count = 1;
            } else {
                last_count += 1;
            }
        }
        if last_count > 0 {
            v.push((last, last_count));
        }
        v
    }
    fn map_hand(hand: String) -> [i32; 5] {
        let mut v = [0; 5];
        for i in hand.as_bytes() {
            let c = match *i as char {
                'R' => 0,
                'Y' => 1,
                'B' => 2,
                'G' => 3,
                'W' => 4,
                _ => panic!("not possible"),
            };
            v[c] += 1;
        }
        v
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let t = Solution::find_min_step("WWRRBBWW".into(), "WRBRW".into());
        assert_eq!(t, 2);
        //"WRRBBW", "RB" -1
        let t = Solution::find_min_step("WRRBBW".into(), "RB".into());
        assert_eq!(t, -1);
        //"G", "GGGGG"  2
        let t = Solution::find_min_step("G".into(), "GGGGG".into());
        assert_eq!(t, 2);
        // "RBYYBBRRB", "YRBGB" ,3
        let t = Solution::find_min_step("RBYYBBRRB".into(), "YRBGB".into());
        assert_eq!(t, 3);

        //        "BGGRRYY" "BBYRG" 5
        let t = Solution::find_min_step("BGGRRYY".into(), "BBYRG".into());
        assert_eq!(t, 5);
    }
}
