/*
288. 单词的唯一缩写

一个单词的缩写需要遵循 <起始字母><中间字母数><结尾字母> 这样的格式。

以下是一些单词缩写的范例：

a) it                      --> it    (没有缩写)

     1
     ↓
b) d|o|g                   --> d1g

              1    1  1
     1---5----0----5--8
     ↓   ↓    ↓    ↓  ↓
c) i|nternationalizatio|n  --> i18n

              1
     1---5----0
↓   ↓    ↓
d) l|ocalizatio|n          --> l10n
假设你有一个字典和一个单词，请你判断该单词的缩写在这本字典中是否唯一。若单词的缩写在字典中没有任何 其他 单词与其缩写相同，则被称为单词的唯一缩写。

示例：

给定 dictionary = [ "deer", "door", "cake", "card" ]

isUnique("dear") -> false
isUnique("cart") -> true
isUnique("cane") -> false
isUnique("make") -> true

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/unique-word-abbreviation
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
考虑到字典中本身可能存在重复的单词问题
*/
use std::collections::HashMap;

struct ValidWordAbbr {
    m: HashMap<String, Vec<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut m = HashMap::new();
        let mut dup = HashMap::new();
        let mut dictionary = dictionary;
        for s in dictionary {
            let e = dup.entry(s.clone()).or_insert(false);
            if *e {
                continue;
            }
            *e = true;
            let abbv = Self::abbv(&s);
            m.entry(abbv).or_insert(Vec::new()).push(s);
        }
        ValidWordAbbr { m }
    }
    fn abbv(s: &String) -> String {
        if s.len() <= 2 {
            return s.clone();
        }
        let mut v = Vec::new();
        let s = s.as_bytes();
        v.push(s[0]);
        let l = format!("{}", s.len() - 2);
        l.as_bytes().iter().for_each(|n| {
            v.push(*n);
        });
        v.push(s[s.len() - 1]);
        unsafe { String::from_utf8_unchecked(v) }
    }
    fn is_unique(&self, word: String) -> bool {
        let abbv = Self::abbv(&word);
        let e = self.m.get(&abbv);
        match e {
            None => true,
            Some(v) => {
                if v.len() > 1 {
                    return false;
                } else {
                    if word == *v.get(0).expect("must have one") {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
    }
}

/**
 * Your ValidWordAbbr object will be instantiated and called as such:
 * let obj = ValidWordAbbr::new(dictionary);
 * let ret_1: bool = obj.is_unique(word);
 */
#[cfg(test)]
mod test {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        let t = ValidWordAbbr::new(
            vec!["deer", "door", "cake", "card"]
                .iter()
                .map(|n| String::from(*n))
                .collect(),
        );
        /*
                isUnique("dear") -> false
        isUnique("cart") -> true
        isUnique("cane") -> false
        isUnique("make") -> true
        */
        assert_eq!(t.is_unique(String::from("dear")), false);
        assert_eq!(t.is_unique(String::from("cart")), true);
        assert_eq!(t.is_unique(String::from("cane")), false);
        assert_eq!(t.is_unique(String::from("make")), true);
    }
}
