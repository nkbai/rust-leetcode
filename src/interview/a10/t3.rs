struct Solution {}
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let ss = s.as_bytes();
        let mut count = [0; 4];
        for s in ss {
            match *s as char {
                'Q' => count[0] += 1,
                'W' => count[1] += 1,
                'E' => count[2] += 1,
                'R' => count[3] += 1,
                _ => panic!("impossible"),
            }
        }
        let avg = ss.len() as i32 / 4;
        let mut total = 0;
        for i in 0..4 {
            if count[i] > avg {
                total += count[i] - avg;
            }
        }
        total
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::balanced_string("WWEQERQWQWWRWWERQWEQ".into()), 4);
    }
}
