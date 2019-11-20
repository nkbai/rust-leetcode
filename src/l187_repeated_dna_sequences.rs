/*
187. 重复的DNA序列

所有 DNA 都由一系列缩写为 A，C，G 和 T 的核苷酸组成，例如：“ACGAATTCCG”。在研究 DNA 时，识别 DNA 中的重复序列有时会对研究非常有帮助。

编写一个函数来查找 DNA 分子中所有出现超过一次的 10 个字母长的序列（子串）。



示例：

输入：s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
输出：["AAAAACCCCC", "CCCCCAAAAA"]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/repeated-dna-sequences
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
注意题目中说了10个字母长的序列,长度是固定的
所以非常简单,10个字符是个字符统计即可,建立一个hashMap,
字符串长度为N,那么就会出现10*N个元素,统计哪些出现次数大于1即可
*/

use std::cmp::{Ordering, Reverse};
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashMap;

struct Solution;
///注释掉的这部分是如果你想自定义一个数据结构,支持Sort
/// 那么就必须实现这些,实现起来真别扭,尤其是设计trait的继承时,
//#[derive(  Eq, Ord)]
struct PosAndString(usize, String);
//impl PartialEq for CountAndPos{
//    fn eq(&self, other: &Self) -> bool {
//        return false;
//    }
//}
//impl PartialOrd for CountAndPos{
//    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//        let r= self.0.partial_cmp(&self.0);
//        match r{
//            None=>{
//                None
//            }
//            Some(r)=>{
//                match r{
//                    Ordering::Less=>Some(Ordering::Greater),
//                    Ordering::Greater=>Some(Ordering::Less),
//                    Ordering::Equal=>Some(Ordering::Equal),
//                }
//            }
//        }
//    }
//}
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut m = HashMap::new();
        let s = s.as_bytes();
        if s.len() < 10 {
            return Vec::new();
        }
        for i in 0..s.len() - 10+1 {
            let r = i..(i + 10);
            let key = &s[i..(i + 10)];
            m.entry(key).or_insert((0, i)).0 += 1;
        }
        let mut res = Vec::new();
        let mut res2 = Vec::new();

        //这里需要排序,按照第一次出现的位置进行排序才行
        for (k, v) in m {
            if v.0 > 1 {
                res2.push(PosAndString(v.1, unsafe {
                    String::from_utf8_unchecked(Vec::from(k))
                }));
            }
        }
//        res2.sort();
        res2.sort_by(|a,b|{
           a.0.cmp(&b.0)
        });
        for x in res2 {
            res.push(x.1);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::find_repeated_dna_sequences(String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT")),
            vec![String::from("AAAAACCCCC"), String::from("CCCCCAAAAA")]
        );
    }
}
