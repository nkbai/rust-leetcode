/*
31. 下一个排列
实现获取下一个排列的函数，算法需要将给定数字序列重新排列成字典序中下一个更大的排列。

如果不存在下一个更大的排列，则将数字重新排列成最小的排列（即升序排列）。

必须原地修改，只允许使用额外常数空间。

以下是一些例子，输入位于左侧列，其相应输出位于右侧列。
1,2,3 → 1,3,2
3,2,1 → 1,2,3
1,1,5 → 1,5,1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/next-permutation
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
比如 6 3 5 4 2 1
下一个应该是
6 4 1 2 3 5
拐点在最后一个升序到降序部分,也就是
3,5,4 中的5
那么降序部分调整为升序
然后跳出一个大于3的数和他交换即可.

*/
struct Solution {}
impl Solution {
    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let t = nums[i];
        nums[i] = nums[j];
        nums[j] = t;
    }
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let l = nums.len();
        //长度1,2的情况特殊处理一下
        match nums.len() {
            1 => {
                return;
            }
            2 => {
                Self::swap(nums, 0, 1);
                return;
            }
            _ => {}
        }
        if nums[l - 1] > nums[l - 2] {
            Self::swap(nums, l - 1, l - 2); //最后一个是升序,那很简单,直接交换即可
            return;
        } else {
            //降序,找到上升的位置
            let mut pos = 0;
            for i in (1..l - 1).rev() {
                //不包含l-1的
                if nums[i] > nums[i - 1] {
                    //找到了升序的位置
                    pos = i;
                    break;
                }
            }
            //如果没有找到,那么默认的pos是0,也就是整体倒序.
            Self::reverse(nums, pos, l - 1);
            if pos == 0 {
                return;
            }
            //            println!("pos={},nums={:?}", pos, nums);
            //从pos到l-1至少有一个数大于nums[pos-1],找到最小的那个,和pos-1交换
            for i in pos..l {
                if nums[i] > nums[pos - 1] {
                    Self::swap(nums, i, pos - 1);
                    break;
                }
            }
            //            println!("after swap={:?}", nums);
        }
    }
    //把i到j之间的数倒过来
    fn reverse(nums: &mut Vec<i32>, i: usize, j: usize) {
        let mut i = i;
        let mut j = j;
        while i < j {
            Self::swap(nums, i, j);
            i += 1;
            j -= 1;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        //        let mut v = vec![1, 2, 3];
        //        let t = Solution::next_permutation(&mut v);
        //        assert_eq!(v, vec![1, 3, 2]);
        //        let mut v = vec![3, 2, 1];
        //        let t = Solution::next_permutation(&mut v);
        //        assert_eq!(v, vec![1, 2, 3]);
        let mut v = vec![6, 3, 5, 4, 2, 1];
        let _t = Solution::next_permutation(&mut v);
        assert_eq!(v, vec![6, 4, 1, 2, 3, 5]);
    }
}
