
struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        //负数肯定不是回文数
        if x < 0 {
            return false;
        }
        //个位数肯定是回文数
        if x < 10 {
            return true;
        }
        let mut x = x;
        let mut v = Vec::new();
        while x != 0 {
            let y = x % 10;
            v.push(y);
            x = x / 10;
        }
        let e = v.len() / 2;
        let l = v.len() - 1;
        for i in 0..e {
            if v[i] != v[l - i] {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(1), true);
    }
}
