#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
399. 除法求值
给出方程式 A / B = k, 其中 A 和 B 均为代表字符串的变量， k 是一个浮点型数字。根据已知方程式求解问题，并返回计算结果。如果结果不存在，则返回 -1.0。

示例 :
给定 a / b = 2.0, b / c = 3.0
问题: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
返回 [6.0, 0.5, -1.0, 1.0, -1.0 ]

输入为: vector<pair<string, string>> equations, vector<double>& values, vector<pair<string, string>> queries(方程式，方程式结果，问题方程式)， 其中 equations.size() == values.size()，即方程式的长度与方程式结果长度相等（程式与结果一一对应），并且结果值均为正数。以上为方程式的描述。 返回vector<double>类型。

基于上述例子，输入如下：
```
equations(方程式) = [ ["a", "b"], ["b", "c"] ],
values(方程式结果) = [2.0, 3.0],
queries(问题方程式) = [ ["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"] ].
输入总是有效的。你可以假设除法运算中不会出现除数为0的情况，且不存在任何矛盾的结果。
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/evaluate-division
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
a/b b/c c/d  b/e
如果画成一棵树
```
          a
         /
        b
       /  \
      c   e
     /
    d
```
我们可以以任意一个节点为根,转换成只有一层的树,并且可以计算出他们之间的除法关系
比如a/d=a/b*b/c*c/d
有了简化后的一层树关系,
那么求任意两个数的除法关系,比如求d/e=(a/e)/(a/d)即可

这么考虑到话,按照并查集的思路来做,就会非常简单.

*/
struct Solution {}
#[derive(Debug)]
struct DSU {
    pre: Vec<(usize, f64)>, //第一个数是parent,第二个数则是parent/自己
}
impl DSU {
    //对于没有出现过的字符，无论如何都是-1，就是是x/x，也是-1
    fn find(&mut self, x: usize) -> (usize, f64) {
        if self.pre[x].0 == x {
            return self.pre[x];
        }
        // let prex = self.pre[x];
        let oldprex = self.pre[x];
        let prex = self.find(oldprex.0);
        //因为递归,这里会把一串上面的所有路径都压缩了,
        self.pre[x] = (prex.0, prex.1 * oldprex.1);
        return self.pre[x];
    }
    //返回false,说明x,y在同一个集合里
    fn merge(&mut self, x: usize, y: usize, v: f64) -> bool {
        let mut prex = self.find(x);
        let mut prey = self.find(y);
        if prex.1 == -1.0 {
            prex.1 = 1.0;
            self.pre[x] = (x, 1.0);
        }
        if prey.1 == -1.0 {
            prey.1 = 1.0;
            self.pre[y] = (y, 1.0);
        }
        if prex.0 == prey.0 {
            return false;
        }
        /*
        prex/x=a
        prey/y=b
        prey/prex *(x/y)=b/a
        prey/prex=(b/a)*(y/x)
        */
        //注意这里是设置的是prex的parent,而不是x的parent
        let parent = prey.0;
        let parentv = prey.1 / prex.1 * (1.0 / v);
        self.pre[prex.0] = (parent, parentv);
        return true;
    }
}
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut dsu = DSU { pre: vec![] };
        let mut r = vec![];
        for i in 0..255 {
            dsu.pre.push((i, -1.0));
        }
        for (i, e) in equations.iter().enumerate() {
            let a = e[0].as_str().chars().nth(0).unwrap() as usize;
            let b = e[1].as_str().chars().nth(0).unwrap() as usize;
            dsu.merge(a, b, values[i]);
        }
        for q in queries.iter() {
            let a = q[0].as_str().chars().nth(0).unwrap() as usize;
            let b = q[1].as_str().chars().nth(0).unwrap() as usize;
            let a = dsu.find(a);
            let b = dsu.find(b);
            if a.0 != b.0 {
                //不在同一个集合里，结果只能是-1.0
                r.push(-1.0);
            } else {
                //求7/5
                //a/7=? a/5=? 求7/5
                //b.1=m/b a.1=m/a
                if a.1 == -1.0 {
                    r.push(-1.0); //没有出现过的表达式，都是-1
                } else {
                    let a_div_b = b.1 / a.1;
                    r.push(a_div_b);
                }
            }
        }
        r
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dsu1() {
        let mut dsu = DSU { pre: vec![] };
        for i in 0..255 {
            dsu.pre.push((i, 1.0));
        }
        dsu.merge(3, 5, 2.0);
        dsu.merge(7, 3, 4.0);
        // println!("dus={:?}", dsu);
        let v = dsu.find(5);
        assert_eq!(v.0, 5);
        assert_eq!(v.1, 1.0);
        let v1 = dsu.find(7);
        assert_eq!(v1.0, 5);
        assert_eq!(v1.1, 0.125);

        //求7/5
        //a/7=? a/5=? 求7/5
        let a75 = v.1 / v1.1;
        assert_eq!(a75, 8.0);
    }
    #[test]
    fn test_dsu2() {
        let mut dsu = DSU { pre: vec![] };
        for i in 0..255 {
            dsu.pre.push((i, 1.0));
        }
        dsu.merge(3, 5, 2.0);
        dsu.merge(3, 7, 4.0);
        // println!("dus={:?}", dsu);
        let v = dsu.find(5);
        assert_eq!(v.0, 7);
        assert_eq!(v.1, 0.5);
        let v1 = dsu.find(7);
        assert_eq!(v1.0, 7);
        assert_eq!(v1.1, 1.0);

        //求7/5
        //a/7=? a/5=? 求7/5
        let a75 = v.1 / v1.1;
        assert_eq!(a75, 0.5);
    }
    #[test]
    fn test() {
        let equations: Vec<Vec<String>> =
            vec![vec!["a".into(), "b".into()], vec!["b".into(), "c".into()]];
        let values = vec![2.0, 3.0];
        let queires: Vec<Vec<String>> = vec![
            vec!["a".into(), "c".into()],
            vec!["b".into(), "a".into()],
            vec!["a".into(), "e".into()],
            vec!["a".into(), "a".into()],
            vec!["x".into(), "x".into()],
        ];
        let r = Solution::calc_equation(equations, values, queires);
        assert_eq!(r, vec![6.0, 0.5, -1.0, 1.0, -1.0]);
    }
}
