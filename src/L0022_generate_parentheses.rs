/*
22. 括号生成
给出 n 代表生成括号的对数，请你写出一个函数，使其能够生成所有可能的并且有效的括号组合。

例如，给出 n = 3，生成结果为：

[
  "((()))",
  "(()())",
  "(())()",
  "()(())",
  "()()()"
]
*/
/*
思路:
用left:表示剩下的左括号个数
用right表示剩下的右括号个数
碰到left==right,只能消耗左括号,否则可以尝试左右
left是不可能大于right的,这种情况肯定不会出现匹配的结果
直到最终消耗完毕
*/
struct Solution {}

impl Solution {
    fn generate_internal(left: i32, right: i32, cur: &Vec<u8>, res: &mut Vec<String>) {
        if left == right && left == 0 {
            res.push(unsafe { String::from_utf8_unchecked(cur.clone()) });
            return;
        }
        if left < 0 || right < 0 {
            panic!("impossible")
        }
        if left > 0 {
            let mut cur = cur.clone();
            cur.push('(' as u8);
            Self::generate_internal(left - 1, right, &cur, res);
        }
        //left最多和right相等还是一个正确的状态,如果left>right,说明出错了.
        if left < right {
            let mut cur = cur.clone();
            cur.push(')' as u8);
            Self::generate_internal(left, right - 1, &cur, res);
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut left = n;
        let mut right = n;
        let mut res = Vec::new();
        let mut cur = Vec::new();
        Self::generate_internal(left, right, &cur, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let t = Solution::generate_parenthesis(1);
        println!("t={:?}", t);
        let t = Solution::generate_parenthesis(2);
        println!("t={:?}", t);
        let t = Solution::generate_parenthesis(3);
        println!("t={:?}", t);
    }
}
