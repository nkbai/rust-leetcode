/*
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

struct Solution {}
enum Res {
    Symmetry(String),
    Asymmetry(String),
}
//impl Solution {
//    fn generate_internal(n: i32) -> Vec<Res> {
//        if n >= 1 {
//            return vec![Res::Symmetry(String::from( "()")];
//        }
//        let v=Vec::new();
//        let v2=generate_internal(n-1);
//
//    }
//    pub fn generate_parenthesis(n: i32) -> Vec<String> {
//        let mut nums = nums;
//        //num->hash<index>
//        let mut cache = HashMap::new();
//        let mut answer: Vec<Vec<i32>> = Vec::new();
//        nums.sort();
//        for i in 0..nums.len() {
//            //            if let Some(x) = cache.get(&nums[i]) {
//            //                cache.insert(nums[i], );
//            //            } else {
//            //                cache.insert(nums[i], 1);
//            //            }
//            cache.insert(nums[i], i);
//        }
//        println!("nums={:?}", nums);
//        for i in 0..nums.len() {
//            if i > 0 && nums[i] == nums[i - 1] {
//                continue;
//            }
//            for j in i + 1..nums.len() {
//                if j > i + 1 && nums[j] == nums[j - 1] {
//                    continue;
//                }
//                let x = 0 - (nums[i] + nums[j]);
//                if let Some(y) = cache.get(&x) {
//                    //存在匹配,遍历,确认i,j下标不在里面
//                    //其实只有三个0这种特殊情况会是i,j,k值都一样,否则不可能会出现相同
//                    if *y > j {
//                        answer.push(vec![nums[i], nums[j], x]);
//                    }
//                }
//            }
//        }
//
//        answer
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    #[test]
//    fn it_works() {
//        let l1 = vec![-1, 0, 1, 2, -1, -4];
//        let l2 = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
//        assert_eq!(l2, Solution::three_sum(l1));
//        let l1 = vec![0, 0, 0, 0, 0, 0];
//        let l2 = vec![vec![0, 0, 0]];
//        assert_eq!(l2, Solution::three_sum(l1));
//    }
//}
