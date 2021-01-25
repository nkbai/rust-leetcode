#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
1353. 最多可以参加的会议数目
给你一个数组 events，其中 events[i] = [startDayi, endDayi] ，表示会议 i 开始于 startDayi ，结束于 endDayi 。

你可以在满足 startDayi <= d <= endDayi 中的任意一天 d 参加会议 i 。注意，一天只能参加一个会议。

请你返回你可以参加的 最大 会议数目。



示例 1：



输入：events = [[1,2],[2,3],[3,4]]
输出：3
解释：你可以参加所有的三个会议。
安排会议的一种方案如上图。
第 1 天参加第一个会议。
第 2 天参加第二个会议。
第 3 天参加第三个会议。
示例 2：

输入：events= [[1,2],[2,3],[3,4],[1,2]]
输出：4
示例 3：

输入：events = [[1,4],[4,4],[2,2],[3,4],[1,1]]
输出：4
示例 4：

输入：events = [[1,100000]]
输出：1
示例 5：

输入：events = [[1,1],[1,2],[1,3],[1,4],[1,5],[1,6],[1,7]]
输出：7


提示：

1 <= events.length <= 10^5
events[i].length == 2
1 <= events[i][0] <= events[i][1] <= 10^5

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-number-of-events-that-can-be-attended
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
贪心算法:
优先选择结束时间早的,然后优先选择时间短的会议,如果长度相同,那么优先选择开始早的会议

时间复杂度:O(NlogN)
空间复杂度:O(N)

正确性如何证明呢?
*/
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
struct Range {
    start: i32,
    end: i32,
}
impl std::cmp::Eq for Range {}
impl std::cmp::PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl std::cmp::PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
//因为是最大堆,所以必须反着来
impl std::cmp::Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.end < other.end {
            Ordering::Greater
        } else if other.end < self.end {
            Ordering::Less
        } else {
            let range1 = self.end - self.start;
            let range2 = other.end - other.start;
            if range1 < range2 {
                Ordering::Greater
            } else if range2 < range1 {
                Ordering::Less
            } else {
                //相等,比较谁的起始时间
                if self.start < other.start {
                    Ordering::Greater
                } else if other.start < self.start {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        };
    }
}
struct Solution {}
impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        for e in events {
            heap.push(Range {
                start: e[0],
                end: e[1],
            });
        }
        let mut m = BTreeMap::new();
        let mut count = 0;
        while let Some(r) = heap.pop() {
            println!("r={:?}", r);
            for i in r.start..r.end + 1 {
                use std::collections::btree_map::Entry;
                let e = m.entry(i);
                match e {
                    Entry::Occupied(_) => {
                        continue;
                    }
                    Entry::Vacant(o) => {
                        o.insert(1);
                        count += 1;
                        break;
                    }
                }
            }
        }
        count
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::max_events(vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![3, 4]]);
        assert_eq!(t, 4);
        // return;
        let t = Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
        assert_eq!(t, 3);
        // return;
        let t = Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]);
        assert_eq!(t, 4);
        let t = Solution::max_events(vec![
            vec![1, 4],
            vec![4, 4],
            vec![2, 2],
            vec![3, 4],
            vec![1, 1],
        ]);
        assert_eq!(t, 4);
        let t = Solution::max_events(vec![vec![1, 100000]]);
        assert_eq!(t, 1);
        let t = Solution::max_events(vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![1, 5],
            vec![1, 6],
            vec![1, 7],
        ]);
        assert_eq!(t, 7);
    }
}
