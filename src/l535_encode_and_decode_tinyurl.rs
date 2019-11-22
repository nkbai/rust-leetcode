/*
535. TinyURL 的加密与解密
TinyURL是一种URL简化服务， 比如：当你输入一个URL https://leetcode.com/problems/design-tinyurl 时，它将返回一个简化的URL http://tinyurl.com/4e9iAk.

要求：设计一个 TinyURL 的加密 encode 和解密 decode 的方法。你的加密和解密算法如何设计和运作是没有限制的，你只需要保证一个URL可以被加密成一个TinyURL，并且这个TinyURL可以用解密方法恢复成原本的URL。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/encode-and-decode-tinyurl
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路
可以有很多种方法,题解中基本上一一列举了
基本上最好的方法就是随机数
*/

use std::collections::HashMap;

struct Solution {
    m: HashMap<String, String>,
    s: Vec<u8>,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            m: HashMap::new(),
            s: String::from("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
                .as_bytes()
                .into(),
        }
    }
    fn getKey(&self) -> String {
        let mut s = Vec::new();
        for _ in 0..6 {
            let mut x: usize = rand::random();
            x = x % self.s.len();
            s.push(self.s[x]);
        }
        unsafe { String::from_utf8_unchecked(s) }
    }
    pub fn encode(&mut self, long_url: String) -> String {
        let mut key = self.getKey();
        while self.m.contains_key(&key) {
            key = self.getKey();
        }
        self.m.insert(key.clone(), long_url);
        return key;
    }
    pub fn decode(&self, short_url: String) -> String {
        match self.m.get(&short_url) {
            None => String::new(),
            Some(x) => x.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reverse_list() {
        let str = String::from("aaabbb");
        let mut s = Solution::new();
        let e = s.encode(str.clone());
        assert_eq!(s.decode(e), str);
    }
}
