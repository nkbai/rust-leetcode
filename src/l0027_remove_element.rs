struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index: usize = 0;
        let mut cnt = 0;
        loop {
            if index >= nums.len() {
                break;
            }
            if nums.get(index).unwrap() == &val {
                nums.remove(index);
                continue;
            }
            index += 1;
            cnt += 1;
        }
        return cnt;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(2, Solution::remove_element(&mut nums, 3));
        assert_eq!(vec![2, 2], nums);
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(5, Solution::remove_element(&mut nums, 2));
        assert_eq!(vec![0, 1, 3, 0, 4], nums);
        let mut nums = vec![2, 2, 2];
        assert_eq!(0, Solution::remove_element(&mut nums, 2));
        //        assert_eq!(Vec::new(), nums);
    }
}
