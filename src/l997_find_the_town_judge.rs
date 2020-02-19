/*
在一个小镇里，按从 1 到 N 标记了 N 个人。传言称，这些人中有一个是小镇上的秘密法官。

如果小镇的法官真的存在，那么：

小镇的法官不相信任何人。
每个人（除了小镇法官外）都信任小镇的法官。
只有一个人同时满足属性 1 和属性 2 。
给定数组 trust，该数组由信任对 trust[i] = [a, b] 组成，表示标记为 a 的人信任标记为 b 的人。

如果小镇存在秘密法官并且可以确定他的身份，请返回该法官的标记。否则，返回 -1。



示例 1：

输入：N = 2, trust = [[1,2]]
输出：2
示例 2：

输入：N = 3, trust = [[1,3],[2,3]]
输出：3
示例 3：

输入：N = 3, trust = [[1,3],[2,3],[3,1]]
输出：-1
示例 4：

输入：N = 3, trust = [[1,2],[2,3]]
输出：-1
示例 5：

输入：N = 4, trust = [[1,3],[1,4],[2,3],[2,4],[4,3]]
输出：3


提示：

1 <= N <= 1000
trust.length <= 10000
trust[i] 是完全不同的
trust[i][0] != trust[i][1]
1 <= trust[i][0], trust[i][1] <= N
*/
struct Solution {}
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut judge = 0;
        let mut cnt_trust: Vec<i32> = (0..n).map(|_| 0).collect();
        let mut trust_other: Vec<bool> = (0..n).map(|_| false).collect();
        for v in trust {
            cnt_trust[(v[1] - 1) as usize] += 1;
            trust_other[(v[0] - 1) as usize] = true;
            if cnt_trust[(v[1] - 1) as usize] == n - 1 && !trust_other[(v[1] - 1) as usize] {
                judge = v[1] - 1;
            }
            if judge >= 0 && trust_other[judge as usize] {
                judge = -1;
            }
        }
        if judge < 0 {
            return -1;
        }
        return judge + 1;
    }
    pub fn find_judge2(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut p1 = vec![0; n as usize + 1];
        let mut p2 = vec![0; n as usize + 1];

        for item in trust {
            p1[item[0] as usize] += 1;
            p2[item[1] as usize] += 1;
        }

        for i in 1..=(n as usize) {
            if p1[i] == 0 && p2[i] == n - 1 {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_find_judge() {
        assert_eq!(2, Solution::find_judge(2, vec![vec![1, 2]]));
        assert_eq!(3, Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]));
        assert_eq!(
            -1,
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]])
        );
        assert_eq!(-1, Solution::find_judge(3, vec![vec![1, 2], vec![2, 3]]));
        assert_eq!(
            3,
            Solution::find_judge(
                4,
                vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
            )
        );
        assert_eq!(1, Solution::find_judge(1, vec![]));
        assert_eq!(
            -1,
            Solution::find_judge2(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]])
        );
    }
}
