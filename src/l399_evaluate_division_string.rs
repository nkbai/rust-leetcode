use std::collections::HashMap;
use std::rc::Rc;

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
    pre: HashMap<Rc<String>, (Rc<String>, f64)>, //第一个数是parent,第二个数则是parent/自己
}
impl DSU {
    //对于没有出现过的字符，无论如何都是-1，就是是x/x，也是-1
    fn find(&mut self, x: Rc<String>) -> (Rc<String>, f64) {
        let pre = match self.pre.get_mut(&x) {
            None => {
                return (x, -1.0);
            }
            Some(p) => p,
        };
        if pre.0 == x {
            return pre.clone();
        }
        // let prex = self.pre[x];
        let oldprex = pre.clone();
        let prex = self.find(oldprex.0.clone());
        let newv = (prex.0.clone(), prex.1 * oldprex.1);
        self.pre.insert(x, newv.clone()).unwrap();
        newv
    }
    fn make_sure_exist(&mut self, x: Rc<String>) {
        self.pre.entry(x.clone()).or_insert((x, 1.0));
    }
    //返回false,说明x,y在同一个集合里
    fn merge(&mut self, x: Rc<String>, y: Rc<String>, v: f64) -> bool {
        self.make_sure_exist(x.clone());
        self.make_sure_exist(y.clone());
        let prex = self.find(x);
        let prey = self.find(y);
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
        self.pre.insert(prex.0, (parent, parentv));
        return true;
    }
}
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut equations = equations;
        let mut queries = queries;
        let mut dsu = DSU {
            pre: HashMap::new(),
        };
        let mut r = vec![];

        let mut n = equations.len() - 1;
        loop {
            let mut e = equations.pop().unwrap();
            let b = e.pop().unwrap();
            let a = e.pop().unwrap();
            dsu.merge(Rc::new(a), Rc::new(b), values[n]);
            if n == 0 {
                //overflow 很恶心人啊
                break;
            }
            n -= 1;
        }
        while !queries.is_empty() {
            let mut q = queries.remove(0);
            let b = q.pop().unwrap();
            let a = q.pop().unwrap();
            let a = dsu.find(Rc::new(a));
            let b = dsu.find(Rc::new(b));
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
        let mut dsu = DSU {
            pre: HashMap::new(),
        };
        dsu.merge(Rc::new("a3".into()), Rc::new("a5".into()), 2.0);
        dsu.merge(Rc::new("a7".into()), Rc::new("a3".into()), 4.0);
        // println!("dus={:?}", dsu);
        let v = dsu.find(Rc::new("a5".into()));
        assert_eq!(v.0, Rc::new("a5".into()));
        assert_eq!(v.1, 1.0);
        let v1 = dsu.find(Rc::new("a7".into()));
        assert_eq!(v1.0, Rc::new("a5".into()));
        assert_eq!(v1.1, 0.125);

        //求7/5
        //a/7=? a/5=? 求7/5
        let a75 = v.1 / v1.1;
        assert_eq!(a75, 8.0);
    }
    #[test]
    fn test_dsu2() {
        let mut dsu = DSU {
            pre: HashMap::new(),
        };
        dsu.merge(Rc::new("a3".into()), Rc::new("a5".into()), 2.0);
        dsu.merge(Rc::new("a3".into()), Rc::new("a7".into()), 4.0);
        // println!("dus={:?}", dsu);
        let v = dsu.find(Rc::new("a5".into()));
        assert_eq!(v.0, Rc::new("a7".into()));
        assert_eq!(v.1, 0.5);
        let v1 = dsu.find(Rc::new("a7".into()));
        assert_eq!(v1.0, Rc::new("a7".into()));
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
