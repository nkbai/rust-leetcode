/*
49. 字母异位词分组
给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。

示例:

输入: ["eat", "tea", "tan", "ate", "nat", "bat"],
输出:
[
  ["ate","eat","tea"],
  ["nat","tan"],
  ["bat"]
]
说明：

所有输入均为小写字母。
不考虑答案输出的顺序。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/group-anagrams
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
这个实际上是将字符串分组,新来一个字符串,看看他和哪个类似,然后放到一组中.
1. 盼到两个字符串是否类似,一种是排序后看是否一致,另一种是统计字符出现的个数
2. 第一种是把排序后的字符串作为map的key,第二种则是把结构体作为map的key
 第二种的结构体可以设计为count[26],其中count[0]表示字符a的个数,count[25]表示字符z的个数
 相比之下,第一种可能会更好一点
 排序是O(KLogK),查找用HashMap是O(1),总体时间复杂度是O(N*KLogK)
 其中N是字符串的个数,K则是某个字符串的长度, 这里没有考虑的是字符串Hash的复杂度,如果一个字符串长度为1万呢?
 至少要遍历一遍,因此复杂度是O(K)
*/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m = HashMap::new();
        for s in strs {
            let mut s2: Vec<_> = s.clone().into_bytes();
            s2.sort();
            //            println!("s={}", s);
            m.entry(s2).or_insert(Vec::new()).push(s);
        }
        //        println!("m={:?}", m);
        let mut v = Vec::new();
        //这里能否避免复制字符串呢?
        for t in m.iter() {
            v.push(t.1.clone());
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = Solution::group_anagrams(
            vec!["eat", "tea", "tan", "nat", "bat", "tan"]
                .iter()
                .map(|n| String::from(*n))
                .collect(),
        );
        println!("t={:?}", t);
    }
}
