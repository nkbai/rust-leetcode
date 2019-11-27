/*
632. 最小区间

你有 k 个升序排列的整数数组。找到一个最小区间，使得 k 个列表中的每个列表至少有一个数包含在其中。

我们定义如果 b-a < d-c 或者在 b-a == d-c 时 a < c，则区间 [a,b] 比 [c,d] 小。

示例 1:

输入:[[4,10,15,24,26], [0,9,12,20], [5,18,22,30]]
输出: [20,24]
解释:
列表 1：[4, 10, 15, 24, 26]，24 在区间 [20,24] 中。
列表 2：[0, 9, 12, 20]，20 在区间 [20,24] 中。
列表 3：[5, 18, 22, 30]，22 在区间 [20,24] 中。
注意:

给定的列表可能包含重复元素，所以在这里升序表示 >= 。
1 <= k <= 3500
-105 <= 元素的值 <= 105
对于使用Java的用户，请注意传入类型已修改为List<List<Integer>>。重置代码模板后可以看到这项改动。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/smallest-range-covering-elements-from-k-lists
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
滑动窗口加优先队列
优先队列中的是k个数组按照最小值的排列
left: 初始空或者不满足覆盖k个数组任意元素的排列
right: 刚好满足覆盖到k个数组任意元素的终点

步骤:
1. left,right为0
2. 从优先队列中取出一个数组,移除最小数,然后放回队列中
3. 将最小数放入计数器中,
4. 检查是否满足条件(覆盖k个数组任意元素的排列)
5. 如果满足,则移动left直到刚好不满足条件,然后计算得到一个区间
6. 回到2,right继续向前走

优先队列在rust中使用binaryHeap实现.
复杂度分析:
队列中的每一个元素都要走两遍,有k个数组,每个数组有m个元素,总数是k*m=N
那么复杂度就是O(Nlogk),每个元素走第一遍的时候,需要对k个数组进行顺序调整,这个是logk,因此总体复杂度就是
O(Nlogk)
空间复杂度:主要是队列占用空间就是O(k),极端情况下可能是O(N)
*/

/*
一开始总是想着优化left向前移动部分,想着在一个logk时间范围内,
使用了BTreeMap等工具,结果处理不了数组中元素相同的情况,徒增麻烦.
*/
struct Solution {}
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Eq, PartialEq)]
struct Element(Vec<i32>, usize); //第一个元素是数组本身,第二个则是他的编号
impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .0
            .get(0)
            .expect("self must have one element")
            .cmp(self.0.get(0).expect("other must have one element"))
    }
}
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut h = BinaryHeap::new();

        let k = nums.len();
        let mut current_max = std::i32::MAX;
        let mut range_start = -1;
        let mut range_end = -1;
        //对应的某个数组是否已经移除了一个数到q中
        let mut k_cnt: Vec<i32> = Vec::with_capacity(nums.len());
        //元素值以及其对应的数组下标
        let mut q: Vec<(usize, i32)> = Vec::new();
        let mut over = Vec::with_capacity(nums.len());

        //初始化队列,放入所有的数组
        for n in nums.iter_mut().enumerate() {
            if n.1.len() <= 0 {
                return vec![range_start, range_end]; //不可能满足条件,因为初始就有数组是空的
            }
            //怎么移出呢?
            h.push(Element(n.1.clone(), n.0));
            k_cnt.push(0);
            over.push(false);
        }
        /*
        1,2,3,4,5,6这些步骤才开始
        */
        let mut current_cnt = 0; //初始k_cnt中已经填充满了有效值
        while h.len() > 0 {
            let mut v = h.pop().expect("h must have one");
            let min = v.0.remove(0);
            let ke = k_cnt.get_mut(v.1).expect("must have");
            if *ke == 0 {
                //第一次找到了这个位置上的元素
                current_cnt += 1;
            }
            *ke += 1;
            q.push((v.1, min));
            if v.0.len() == 0 {
                //某一个数组遍历完了,还不能结束要等到最后一个元素使用完了才能结束
                over[v.1] = true;
            } else {
                h.push(Element(v.0, v.1));
            }
            //已经满足条件,那么移动left,直到
            if current_cnt == k {
                let mut min = q[0];
                let max = q[q.len() - 1];
                while current_cnt == k {
                    let first = q.remove(0);
                    k_cnt[first.0] -= 1;
                    if k_cnt[first.0] == 0 {
                        //某个数组降到了0,再讲就不满足目标了.
                        min = first;
                        current_cnt -= 1;
                    }
                }
                if max.1 - min.1 < current_max {
                    current_max = max.1 - min.1;
                    range_start = min.1;
                    range_end = max.1;
                }
            }
        }

        vec![range_start, range_end]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::smallest_range(vec![vec![10], vec![11]]);
        assert_eq!(t, vec![10, 11]);
        //[[4,10,15,24,26], [0,9,12,20], [5,18,22,30]]
        let t = Solution::smallest_range(vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ]);
        assert_eq!(t, vec![20, 24]);
        /*
        [[-89,1,69,89,90,98],[-43,-36,-24,-14,49,61,66,69],[73,94,94,96],[11,13,76,79,90],[-40,-20,1,9,12,12,14],[-91,-31,0,21,25,26,28,29,29,30],[23,88,89],[31,42,42,57],[-2,6,11,12,12,13,15],[-3,25,34,36,39],[-7,3,29,29,31,32,33],[4,11,14,15,15,18,19],[-34,9,12,19,19,19,19,20],[-26,4,47,53,64,64,64,64,64,65],[-51,-25,36,38,50,54],[17,25,38,38,38,38,40],[-30,12,15,19,19,20,22],[-14,-13,-10,68,69,69,72,74,75],[-39,42,70,70,70,71,72,72,73],[-67,-34,6,26,28,28,28,28,29,30,31]]
        */
    }
}
