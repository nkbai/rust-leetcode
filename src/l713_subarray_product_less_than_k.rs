/*
713. 乘积小于K的子数组
给定一个正整数数组 nums。

找出该数组内乘积小于 k 的连续的子数组的个数。

示例 1:

输入: nums = [10,5,2,6], k = 100
输出: 8
解释: 8个乘积小于100的子数组分别为: [10], [5], [2], [6], [10,5], [5,2], [2,6], [5,2,6]。
需要注意的是 [10,5,2] 并不是乘积小于100的子数组。
说明:

0 < nums.length <= 50000
0 < nums[i] < 1000
0 <= k < 10^6


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/subarray-product-less-than-k
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
注意:
1. 全是正整数
2. 连续
思路:
倒着来,复杂度基本上应该是O(n^2),如果需要优化,那么检索部分可以使用二分查找.
还有一个可优化的地方就是不要用vector的remove,使用范围标志,反正最长不可能超过数组的长度
空间复杂度是O(n)
*/

struct Solution {}
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut cache = Vec::with_capacity(nums.len());
        let mut i = nums.len() - 1;
        let mut cnt = 0;
        loop {
            if nums[i] >= k {
                if i == 0 {
                    break;
                }
                i -= 1;
                cache.clear(); //清除,避免下一轮使用
                continue;
            }
            let cur = nums[i];
            if cur == 1 {
                //1特殊处理
                cnt += cache.len() as i32;
            } else {
                let l = cache.len();
                //新来的数字和旧的序列对比,如果超过100,则停止,注意顺序是从大大小
                let res = cache.binary_search_by(|probe: &i32| match (*probe * cur).cmp(&k) {
                    //不能用除法
                    std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                });
                //如果找到,就应该删除的时候包含pos,否则不包含
                match res {
                    Ok(pos) => {
                        for h in 0..=pos {
                            cache.remove(0);
                        }
                    }
                    Err(pos) => {
                        for h in 0..pos {
                            cache.remove(0);
                        }
                    }
                }
                for k in cache.iter_mut() {
                    *k = *k * cur;
                }
                cnt += cache.len() as i32;
                println!("cache={:?},cur={},cnt={}", cache, cur, cnt);
            }

            cache.push(cur);
            cnt += 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        return cnt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 100, 2, 6], 100),
            4
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![100, 10, 2], 100),
            3
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![100, 100, 200], 100),
            0
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(
                vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3],
                19
            ),
            18
        );
    }
    #[test]
    fn test_num2() {
        let v = vec![10, 8, 3, 1];
        println!(
            "11={:?}",
            v.binary_search_by(|probe| match probe.cmp(&11) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            })
        );
        println!(
            "9={:?}",
            v.binary_search_by(|probe| match probe.cmp(&9) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            })
        );
        println!(
            "8={:?}",
            v.binary_search_by(|probe| match probe.cmp(&8) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            })
        );
        println!(
            "0={:?}",
            v.binary_search_by(|probe| match probe.cmp(&0) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            })
        );
    }
}
