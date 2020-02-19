#[allow(dead_code)]
/*
135. 分发糖果
老师想给孩子们分发糖果，有 N 个孩子站成了一条直线，老师会根据每个孩子的表现，预先给他们评分。

你需要按照以下要求，帮助老师给这些孩子分发糖果：

每个孩子至少分配到 1 个糖果。
相邻的孩子中，评分高的孩子必须获得更多的糖果。
那么这样下来，老师至少需要准备多少颗糖果呢？

示例 1:

输入: [1,0,2]
输出: 5
解释: 你可以分别给这三个孩子分发 2、1、2 颗糖果。
示例 2:

输入: [1,2,2]
输出: 4
解释: 你可以分别给这三个孩子分发 1、2、1 颗糖果。
     第三个孩子只得到 1 颗糖果，这已满足上述两个条件。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/candy
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
一遍遍历法:
1. 首先原则是贪心,尽可能的少给
2. 分数看成从低到高,再从高到低的变化过程
3. 要求最高点要同时满足左右两边的需求
4. 如果碰到相等连续相等的多个数,则等到有变化再参与计算

只需计数单调增和单调减的个数即可,不用关心他们彼此的差距.
特殊情况:
就是连续相等的情况
比如1,2,3,3,3,2,1
 则应该把第一个3视作左边的,最后一个3视作右边的,中间的3都设置为1
*/
use std::cmp::{max, Ordering};

struct Solution {}
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut is_up; //0表示向上增的趋势,1表示向下的趋势
        match ratings.len() {
            0 => {
                return 0;
            }
            1 => {
                return 1;
            }
            2 => {
                if ratings[0] == ratings[1] {
                    return 2;
                } else {
                    return 3;
                }
            }
            _ => {
                if ratings[0] < ratings[1] {
                    is_up = true;
                } else {
                    is_up = false; //不是升序,直接当作降序
                }
            }
        }
        let mut up_count = 0;
        let mut down_count = 0;
        let mut equals = 0;
        let mut last;
        let mut total = 0;
        if is_up {
            last = 0;
        } else {
            last = std::i32::MAX;
        }
        for i in 0..ratings.len() {
            let order = last.cmp(&ratings[i]);
            if order == Ordering::Equal {
                equals += 1;
                continue;
            }
            last = ratings[i];
            //            已经发生了不等,这时候就要考虑想等部分如何处理了
            if equals >= 1 {
                if is_up {
                    let c = up_count * (up_count + 1) / 2; //把第一个相等算在这个趋势中,另一个算在另一个趋势中
                    total += c;
                } else {
                    let m = max(down_count, up_count);
                    let c = down_count * (down_count - 1) / 2;
                    total += c;
                    total += up_count * (up_count - 1) / 2;
                    total += m;
                }
                is_up = order == Ordering::Less;
                if is_up {
                    up_count = 2;
                    down_count = 0;
                } else {
                    down_count = 2;
                    up_count = 0;
                }
                equals -= 1;
                total += equals; //中间的等数都是1
                equals = 0;
                continue;
            }
            equals = 0;
            if is_up == (order == Ordering::Less) {
                if is_up {
                    up_count += 1;
                } else {
                    down_count += 1;
                }
            } else {
                //趋势发生了变化
                if is_up {
                    //由上升变为下降,先不计算,等等
                    down_count += 2;
                } else {
                    //下降变为了上升,开始计算上一波的上升到下降
                    let m = max(up_count, down_count);
                    let c1 = up_count * (up_count - 1) / 2;
                    let c2 = down_count * (down_count - 1) / 2;
                    total += m;
                    total += c1 + c2 - 1; //最后一个个数计入下一个趋势
                    up_count = 2;
                    down_count = 0;
                }
                is_up = !is_up;
            }
        }
        let m = max(down_count, up_count);
        let c1 = up_count * (up_count - 1) / 2;
        let c2 = down_count * (down_count - 1) / 2;
        total += m;
        total += c1 + c2;
        total += equals; //考虑到最后以相等结尾
        total
    }
    #[allow(dead_code)]
    fn need_clear() {
        //        let mut a1 = 0;
        //        let mut a2 = 0;
        //        let mut a3 = 0;
        //        let mut state = 1;
        //        let clear_a1_a2 = || {
        //            a1 = 0;
        //            a2 = 0;
        //        };
        //        let clear_a2_a3 = || {
        //            a2 = 0;
        //            a3 = 0;
        //        };
        //        if state == 1 {
        //            //do something else
        //            clear_a1_a2();
        //        } else if state == 2 {
        //            //do somthing else
        //            clear_a1_a2();
        //        } else {
        //            //do something else
        //            clear_a2_a3();
        //        }
    }
}
#[cfg(test)]
mod test {
    use crate::l135_candy::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::candy(vec![1, 2, 3, 3, 2, 1]), 12);
        assert_eq!(Solution::candy(vec![5, 4, 3, 3, 4, 5]), 12);
        assert_eq!(Solution::candy(vec![5, 4, 3, 3, 3, 3, 4, 5]), 14);
        assert_eq!(Solution::candy(vec![1, 2, 3, 3, 3, 3, 4, 5]), 14);
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 4, 4]), 4);
        assert_eq!(Solution::candy(vec![1, 3, 2, 2, 1]), 7);
    }
}
