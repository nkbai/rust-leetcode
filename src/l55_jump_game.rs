/*
55. 跳跃游戏
给定一个非负整数数组，你最初位于数组的第一个位置。

数组中的每个元素代表你在该位置可以跳跃的最大长度。

判断你是否能够到达最后一个位置。

示例 1:

输入: [2,3,1,1,4]
输出: true
解释: 我们可以先跳 1 步，从位置 0 到达 位置 1, 然后再从位置 1 跳 3 步到达最后一个位置。
示例 2:

输入: [3,2,1,0,4]
输出: false
解释: 无论怎样，你总会到达索引为 3 的位置。但该位置的最大跳跃长度是 0 ， 所以你永远不可能到达最后一个位置。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/jump-game
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
 假设长度为l
1. 从头开始找出第一个能到达终点的位置,也就是a[i]>= l-i-1,假设找到了3
2. 然后将终点设置为3,从1重新开始
直到走到下标0结束
事件复杂度,最糟糕情况下是O(N^2)
*/
struct Solution {}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }
        let mut mark = vec![true; nums.len()];
        Self::jump(&nums, nums.len() - 1, &mut mark)
    }
    fn jump(nums: &Vec<i32>, pos: usize, mark: &mut Vec<bool>) -> bool {
        println!("jump,pos={}", pos);
        for i in 0..pos {
            if nums[i] as usize >= pos - i {
                if i == 0 {
                    return true;
                }
                if Self::jump(nums, i, mark) {
                    return true;
                }
                break;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
        let v = vec![
            2, 0, 6, 9, 8, 4, 5, 0, 8, 9, 1, 2, 9, 6, 8, 8, 0, 6, 3, 1, 2, 2, 1, 2, 6, 5, 3, 1, 2,
            2, 6, 4, 2, 4, 3, 0, 0, 0, 3, 8, 2, 4, 0, 1, 2, 0, 1, 4, 6, 5, 8, 0, 7, 9, 3, 4, 6, 6,
            5, 8, 9, 3, 4, 3, 7, 0, 4, 9, 0, 9, 8, 4, 3, 0, 7, 7, 1, 9, 1, 9, 4, 9, 0, 1, 9, 5, 7,
            7, 1, 5, 8, 2, 8, 2, 6, 8, 2, 2, 7, 5, 1, 7, 9, 6,
        ];
        assert_eq!(false, Solution::can_jump(v));
    }
}
