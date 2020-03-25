#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
990. 等式方程的可满足性

给定一个由表示变量之间关系的字符串方程组成的数组，每个字符串方程 equations[i] 的长度为 4，并采用两种不同的形式之一："a==b" 或 "a!=b"。在这里，a 和 b 是小写字母（不一定不同），表示单字母变量名。

只有当可以将整数分配给变量名，以便满足所有给定的方程时才返回 true，否则返回 false。



示例 1：

输入：["a==b","b!=a"]
输出：false
解释：如果我们指定，a = 1 且 b = 1，那么可以满足第一个方程，但无法满足第二个方程。没有办法分配变量同时满足这两个方程。
示例 2：

输出：["b==a","a==b"]
输入：true
解释：我们可以指定 a = 1 且 b = 1 以满足满足这两个方程。
示例 3：

输入：["a==b","b==c","a==c"]
输出：true
示例 4：

输入：["a==b","b!=c","c==a"]
输出：false
示例 5：

输入：["c==c","b==d","x!=z"]
输出：true


提示：

1 <= equations.length <= 500
equations[i].length == 4
equations[i][0] 和 equations[i][3] 是小写字母
equations[i][1] 要么是 '='，要么是 '!'
equations[i][2] 是 '='

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/satisfiability-of-equality-equations
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

 */

/*
思路:
==具有传递性,!=则没有
先将等等放在并查集中,
然后拿不等来校验,如果都不在一个集合中,则返回true
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
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut not_equals = vec![];
        let mut dsu = DSU::new(26);
        for e in equations {
            let e = e.as_bytes();
            let a = e[0] - 'a' as u8;
            let b = e[3] - 'a' as u8;
            if e[1] == '=' as u8 {
                dsu.merge(a as i32, b as i32);
            } else {
                not_equals.push((a, b));
            }
        }
        for (a, b) in not_equals {
            if dsu.find(a as i32) == dsu.find(b as i32) {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::equations_possible(vec!["a==b".into(), "b==c".into(), "a==c".into()]);
        assert_eq!(t, true);
        let t = Solution::equations_possible(vec!["a==b".into(), "b!=c".into(), "c==a".into()]);
        assert_eq!(t, false);
        let t = Solution::equations_possible(vec!["c==c".into(), "b==d".into(), "x!=z".into()]);
        assert_eq!(t, true);
    }
}
