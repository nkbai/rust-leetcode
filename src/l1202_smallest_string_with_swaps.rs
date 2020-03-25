#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
1202. 交换字符串中的元素

给你一个字符串 s，以及该字符串中的一些「索引对」数组 pairs，其中 pairs[i] = [a, b] 表示字符串中的两个索引（编号从 0 开始）。

你可以 任意多次交换 在 pairs 中任意一对索引处的字符。

返回在经过若干次交换后，s 可以变成的按字典序最小的字符串。



示例 1:

输入：s = "dcab", pairs = [[0,3],[1,2]]
输出："bacd"
解释：
交换 s[0] 和 s[3], s = "bcad"
交换 s[1] 和 s[2], s = "bacd"
示例 2：

输入：s = "dcab", pairs = [[0,3],[1,2],[0,2]]
输出："abcd"
解释：
交换 s[0] 和 s[3], s = "bcad"
交换 s[0] 和 s[2], s = "acbd"
交换 s[1] 和 s[2], s = "abcd"
示例 3：

输入：s = "cba", pairs = [[0,1],[1,2]]
输出："abc"
解释：
交换 s[0] 和 s[1], s = "bca"
交换 s[1] 和 s[2], s = "bac"
交换 s[0] 和 s[1], s = "abc"


提示：

1 <= s.length <= 10^5
0 <= pairs.length <= 10^5
0 <= pairs[i][0], pairs[i][1] < s.length
s 中只含有小写英文字母

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/smallest-string-with-swaps
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。。

 */

/*
思路:
可以交换是具有传递性的,具体来说[0,1],[1,2],那么[0,2]肯定也是成立的.
把可以交换的放到一个集合里,然后针对这个集合按照字母顺序进行排序即可.

涉及到排序,
如果所有的字符都可以兑换,也就是最糟糕的情形,那么复杂度就是O(nlogn),其中n是pairs.length
*/
struct Solution {}
struct DSU {
    pre: Vec<i32>,
}
impl DSU {
    fn new(n: usize) -> DSU {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(i as i32);
        }
        DSU { pre: v }
    }
    fn find(&mut self, x: i32) -> i32 {
        if self.pre[x as usize] == x {
            return x;
        }
        // let prex = self.pre[x];
        let prex = self.find(self.pre[x as usize]);
        //因为递归,这里会把一串上面的所有路径都压缩了,
        self.pre[x as usize] = prex;
        return prex;
    }
    //返回false,说明x,y在同一个集合里
    fn merge(&mut self, x: i32, y: i32) -> bool {
        let prex = self.find(x);
        let prey = self.find(y);
        if prex == prey {
            return false;
        }
        //注意这里是设置的是prex的parent,而不是x的parent
        self.pre[prey as usize] = prex;
        return true;
    }
}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut s_origin = s;
        let s = unsafe { s_origin.as_mut_vec() }.as_mut_slice();
        let mut dsu = DSU::new(s.len());
        for p in pairs {
            dsu.merge(p[0], p[1]);
        }
        let mut collects: Vec<Option<Vec<i32>>> = vec![None; s.len()];
        for i in 0..s.len() as i32 {
            let x = dsu.find(i);
            if x != i {
                let r = collects.get_mut(x as usize).unwrap();

                match r {
                    None => *r = Some(vec![i, x]),
                    Some(ref mut v) => {
                        v.push(i);
                    }
                }
            }
        }
        for c in collects {
            if c.is_none() {
                continue;
            }
            /*
            假设接种是[3,7,9,1] 表示这四个位置对应点字母可以排序
            那么首先位置按照从小到大排序[1,3,7.9]
            然后再对这些位置上的字母['b','d','c','x']进行排序
            然后把排序后的字母按照位置写会原来的s
            */

            let mut c = c.unwrap();
            c.sort();
            let mut c2 = Vec::with_capacity(c.len());
            for i in c.iter() {
                let i = *i;
                c2.push(s[i as usize]);
            }
            c2.sort();
            for (i, j) in c.iter().enumerate() {
                let j = *j as usize;
                s[j] = c2[i];
            }
        }
        s_origin
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::smallest_string_with_swaps("dcab".into(), vec![vec![0, 3], vec![1, 2]]);
        assert_eq!(t, String::from("bacd"));
        let t = Solution::smallest_string_with_swaps(
            "dcab".into(),
            vec![vec![0, 3], vec![1, 2], vec![0, 2]],
        );
        assert_eq!(t, String::from("abcd"));
    }
}
