/*
60. 第k个排列 https://leetcode-cn.com/problems/permutation-sequence/
给出集合 [1,2,3,…,n]，其所有元素共有 n! 种排列。

按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：

"123"
"132"
"213"
"231"
"312"
"321"
给定 n 和 k，返回第 k 个排列。

说明：

给定 n 的范围是 [1, 9]。
给定 k 的范围是[1,  n!]。
示例 1:

输入: n = 3, k = 3
输出: "213"
示例 2:

输入: n = 4, k = 9
输出: "2314"


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/permutation-sequence
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

/*

思路:
以n=3,k=4为例,
那么以1,2,3开头的都是2个,那么很容易确定开头的数字,那么很容易确定开头的第一数字是2
如果k==0,说明就是剩下的这个排列,不用继续了

复杂度分析:
假设其中O(n),n有多少位,最多需要递归多少次

其他:
一开始的时候想着减少递归,比如碰到k%item 等于0的情况特殊处理,因为知道是最大那个,碰到k%item==1情况,
知道是下一个区域的第一个, 但是这会造成代码不必要的复杂度,还容易出问题.
*/
use std::collections::BTreeMap;

struct Solution {}
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        let mut v: Vec<usize> = (1..=n).collect();
        let mut pmap: BTreeMap<usize, usize> = BTreeMap::new();
        pmap.insert(1, 1);
        for i in 2..=n {
            pmap.insert(i, pmap.get(&(i - 1)).unwrap() * i);
        }
        let mut result = Vec::new();
        Solution::search(&pmap, v, k - 1, &mut result);
        let v: Vec<u8> = result.iter().map(|i| (i + '0' as usize) as u8).collect();
        let s: String = String::from_utf8(v).unwrap();
        s
    }
    fn search(
        pmap: &BTreeMap<usize, usize>,
        mut left: Vec<usize>,
        k: usize,
        result: &mut Vec<usize>,
    ) {
        assert!(!left.is_empty(), "left empty");
        if k == 0 {
            result.extend(left);
            return;
        }
        /*
        找出最高位应该是什么
        */
        let item = pmap.get(&(left.len() - 1)).unwrap();
        let mut first = k / *item;
        let mut index = k % *item;
        assert!(!(first >= left.len()), "k is too large");

        //        移除最高位
        let high = left.remove(first);
        result.push(high);
        Solution::search(pmap, left, k - first * *item, result); //继续搜索剩下的
    }
}

mod test {
    use super::*;
    #[test]
    fn test_permute() {
        assert_eq!("1", Solution::get_permutation(1, 1));
        assert_eq!("12", Solution::get_permutation(2, 1));
        assert_eq!("21", Solution::get_permutation(2, 2));
        assert_eq!("123", Solution::get_permutation(3, 1));
        assert_eq!("132", Solution::get_permutation(3, 2));
        assert_eq!("213", Solution::get_permutation(3, 3));
        assert_eq!("231", Solution::get_permutation(3, 4));
        assert_eq!("312", Solution::get_permutation(3, 5));
        assert_eq!("321", Solution::get_permutation(3, 6));
    }
}
