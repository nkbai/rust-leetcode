use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
面试题 17.07. 婴儿名字
每年，政府都会公布一万个最常见的婴儿名字和它们出现的频率，也就是同名婴儿的数量。有些名字有多种拼法，例如，John 和 Jon 本质上是相同的名字，但被当成了两个名字公布出来。给定两个列表，一个是名字及对应的频率，另一个是本质相同的名字对。设计一个算法打印出每个真实名字的实际频率。注意，如果 John 和 Jon 是相同的，并且 Jon 和 Johnny 相同，则 John 与 Johnny 也相同，即它们有传递和对称性。

在结果列表中，选择字典序最小的名字作为真实名字。

示例：

输入：names = ["John(15)","Jon(12)","Chris(13)","Kris(4)","Christopher(19)"], synonyms = ["(Jon,John)","(John,Johnny)","(Chris,Kris)","(Chris,Christopher)"]
输出：["John(27)","Chris(36)"]
提示：

names.length <= 100000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/baby-names-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

 */

/*
思路:
并查集,
注意merge的时候,选择字典序小的作为parent就行了.

*/
struct Solution {}

#[derive(Debug)]
struct DSU {
    pre: HashMap<Rc<String>, Rc<String>>, //第一个数是parent,第二个数则是parent/自己
}
impl DSU {
    //对于没有出现过的字符，无论如何都是-1，就是是x/x，也是-1
    fn find(&mut self, x: Rc<String>) -> Rc<String> {
        let pre = match self.pre.get(&x) {
            None => {
                return x.clone();
            }
            Some(p) => p,
        };
        // let prex = self.pre[x];
        let old_prex = pre.clone();
        let prex = self.find(old_prex.clone());
        self.pre.insert(x, prex.clone());
        prex
    }
    //返回false,说明x,y在同一个集合里
    fn merge(&mut self, x: Rc<String>, y: Rc<String>) -> bool {
        let prex = self.find(x);
        let prey = self.find(y);
        if prex == prey {
            return false;
        }
        if prex < prey {
            self.pre.insert(prey, prex);
        } else {
            self.pre.insert(prex, prey);
        }
        return true;
    }
}
impl Solution {
    fn parse_names(s: &str) -> (String, i32) {
        let mut name = String::new();
        let mut sz = String::new();
        let mut i = 0;
        for (pos, c) in s.as_bytes().iter().enumerate() {
            let c = *c;
            if c == '(' as u8 {
                i = pos;
                break;
            }
            name.push(c as char);
        }
        //  println!("name={},i={}", name, i);
        for c in s.as_bytes().iter().skip(i + 1) {
            let c = *c;
            if c == ')' as u8 {
                break;
            }
            sz.push(c as char);
        }
        // println!("sz={}", sz);
        let n = sz.parse::<i32>().unwrap();
        (name, n)
    }
    fn parse_synonyms(s: &str) -> (String, String) {
        let mut name1 = String::new();
        let mut name2 = String::new();
        let mut i = 0;
        for (pos, c) in s.as_bytes().iter().skip(1).enumerate() {
            let c = *c;
            if c == ',' as u8 {
                i = pos;
                break;
            }
            name1.push(c as char);
        }
        //,和开始的(
        for c in s.as_bytes().iter().skip(i + 2) {
            let c = *c;
            if c == ')' as u8 {
                break;
            }
            name2.push(c as char);
        }
        (name1, name2)
    }
    pub fn truly_most_popular(names: Vec<String>, synonyms: Vec<String>) -> Vec<String> {
        let mut dsu = DSU {
            pre: HashMap::new(),
        };
        let mut m = HashMap::new();
        for name in names {
            let (name, number) = Self::parse_names(&name);
            let name = Rc::new(name);
            m.insert(name, number);
        }
        for s in synonyms {
            let (name1, name2) = Self::parse_synonyms(&s);
            dsu.merge(Rc::new(name1), Rc::new(name2));
        }
        let mut m2 = HashMap::new();
        for (name, number) in m {
            let p = dsu.find(name);
            *m2.entry(p).or_insert(0) += number;
        }
        let mut v = Vec::new();
        for (name, number) in m2 {
            v.push(format!("{}({})", *name, number));
        }
        v
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let r = Solution::parse_names("John(15)");
        assert_eq!(r.0, "John".to_string());
        assert_eq!(r.1, 15);
        let r = Solution::parse_synonyms("(John,Johnny)");
        assert_eq!(r.0, "John".to_string());
        assert_eq!(r.1, "Johnny".to_string());
    }
    #[test]
    fn test() {
        let r = Solution::truly_most_popular(
            vec![
                "John(15)".to_string(),
                "Jon(12)".to_string(),
                "Chris(13)".to_string(),
                "Kris(4)".to_string(),
                "Christopher(19)".to_string(),
            ],
            vec![
                "(Jon,John)".to_string(),
                "(John,Johnny)".to_string(),
                "(Chris,Kris)".to_string(),
                "(Chris,Christopher)".to_string(),
            ],
        );
        println!("r={:?}", r);
    }
}
