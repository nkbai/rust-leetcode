use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
721. 账户合并
给定一个列表 accounts，每个元素 accounts[i] 是一个字符串列表，其中第一个元素 accounts[i][0] 是 名称 (name)，其余元素是 emails 表示该帐户的邮箱地址。

现在，我们想合并这些帐户。如果两个帐户都有一些共同的邮件地址，则两个帐户必定属于同一个人。请注意，即使两个帐户具有相同的名称，它们也可能属于不同的人，因为人们可能具有相同的名称。一个人最初可以拥有任意数量的帐户，但其所有帐户都具有相同的名称。

合并帐户后，按以下格式返回帐户：每个帐户的第一个元素是名称，其余元素是按顺序排列的邮箱地址。accounts 本身可以以任意顺序返回。

例子 1:

Input:
accounts = [["John", "johnsmith@mail.com", "john00@mail.com"], ["John", "johnnybravo@mail.com"], ["John", "johnsmith@mail.com", "john_newyork@mail.com"], ["Mary", "mary@mail.com"]]
Output: [["John", 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com'],  ["John", "johnnybravo@mail.com"], ["Mary", "mary@mail.com"]]
Explanation:
  第一个和第三个 John 是同一个人，因为他们有共同的电子邮件 "johnsmith@mail.com"。
  第二个 John 和 Mary 是不同的人，因为他们的电子邮件地址没有被其他帐户使用。
  我们可以以任何顺序返回这些列表，例如答案[['Mary'，'mary@mail.com']，['John'，'johnnybravo@mail.com']，
  ['John'，'john00@mail.com'，'john_newyork@mail.com'，'johnsmith@mail.com']]仍然会被接受。

注意：

accounts的长度将在[1，1000]的范围内。
accounts[i]的长度将在[1，10]的范围内。
accounts[i][j]的长度将在[1，30]的范围内。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/accounts-merge
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
思路:
和399类似，都是string类型的并查集，

只不过到最后按照parent重新组织一下数据。

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
        self.pre.insert(prex, prey);
        return true;
    }
}
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // let mut accounts = accounts;
        let mut r = vec![];
        let mut dsu = DSU {
            pre: HashMap::new(),
        };
        let mut m = HashMap::new(); //email=>account
                                    //放入并查集中
        for mut account in accounts {
            let a = account.remove(0);
            let a = Rc::new(a);
            let first_email = Rc::new(account.pop().unwrap());
            m.insert(first_email.clone(), a.clone());
            for email in account {
                let email = Rc::new(email);
                dsu.merge(first_email.clone(), email.clone());
                m.insert(email, a.clone());
            }
        }
        let mut m2 = HashMap::new();
        for (email, account) in m.iter() {
            let p = dsu.find(email.clone());
            m2.entry(p)
                .or_insert(vec![(**account).clone()])
                .push((**email).clone());
        }
        for (_, mut v) in m2 {
            v.sort();
            r.push(v);
        }
        r
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dsu() {
        let mut dsu = DSU {
            pre: HashMap::new(),
        };
        assert_eq!(dsu.find(Rc::new("a1".into())), Rc::new("a1".into()));
        let r = dsu.merge(Rc::new("a1".into()), Rc::new("a2".into()));
        assert_eq!(r, true);
        let r = dsu.merge(Rc::new("a3".into()), Rc::new("a2".into()));
        assert_eq!(r, true);
        assert_eq!(dsu.find(Rc::new("a1".into())), Rc::new("a2".into()));
    }
    #[test]
    fn test() {
        let t = Solution::accounts_merge(vec![
            vec![
                "John".into(),
                "johnsmith@mail.com".into(),
                "john00@mail.com".into(),
            ],
            vec!["John".into(), "johnnybravo@mail.com".into()],
            vec![
                "John".into(),
                "johnsmith@mail.com".into(),
                "john_newyork@mail.com".into(),
            ],
            vec!["Mary".into(), "mary@mail.com".into()],
        ]);
        println!("t={:?}", t);
    }
}
