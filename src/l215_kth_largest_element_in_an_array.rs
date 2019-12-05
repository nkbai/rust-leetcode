/*
215. 数组中的第K个最大元素

在未排序的数组中找到第 k 个最大的元素。请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

示例 1:

输入: [3,2,1,5,6,4] 和 k = 2
输出: 5
示例 2:

输入: [3,2,3,1,2,4,5,5,6] 和 k = 4
输出: 4
说明:

你可以假设 k 总是有效的，且 1 ≤ k ≤ 数组的长度。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/kth-largest-element-in-an-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
思路1: 堆排序,但是维持堆的大小再k个,一旦堆中的元素大于k个, 扔掉最小堆顶元素.
思路2:[快速选择算法](https://en.wikipedia.org/wiki/Quickselect)
思路和快速排序是一样的,只不过不对两边都排序,只会对包含k的那半边排序
时间复杂度平均为O(N),因为每次比较的次数是N+N/2+N/4+N/8 ... 极限就是2N
```
 function partition(list, left, right, pivotIndex)
     pivotValue := list[pivotIndex]
     swap list[pivotIndex] and list[right]  // Move pivot to end
     storeIndex := left
     for i from left to right-1
         if list[i] < pivotValue
             swap list[storeIndex] and list[i]
             increment storeIndex
     swap list[right] and list[storeIndex]  // Move pivot to its final place
     return storeIndex

 // Returns the k-th smallest element of list within left..right inclusive
  // (i.e. left <= k <= right).
  function select(list, left, right, k)
     if left = right        // If the list contains only one element,
         return list[left]  // return that element
     pivotIndex  := ...     // select a pivotIndex between left and right,
                            // e.g., left + floor(rand() % (right - left + 1))
     pivotIndex  := partition(list, left, right, pivotIndex)
     // The pivot is in its final sorted position
     if k = pivotIndex
         return list[k]
     else if k < pivotIndex
         return select(list, left, pivotIndex - 1, k)
     else
         return select(list, pivotIndex + 1, right, k)
```
*/
struct Solution {}
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let l = nums.len();
        return Self::select(&mut nums, 0, l - 1, (k - 1) as usize);
    }
    fn select(nums: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
        if left == right {
            return nums[left];
        }

        let pivot = Self::partition(nums, left, right, left); //每次都选择最左边的数,简单起见.
        if k == pivot {
            return nums[k];
        } else if k < pivot {
            return Self::select(nums, left, pivot - 1, k);
        } else {
            return Self::select(nums, pivot + 1, right, k);
        }
    }
    /*
    function partition(list, left, right, pivotIndex)
    pivotValue := list[pivotIndex]
    swap list[pivotIndex] and list[right]  // Move pivot to end
    storeIndex := left
    for i from left to right-1
        if list[i] < pivotValue
            swap list[storeIndex] and list[i]
            increment storeIndex
    swap list[right] and list[storeIndex]  // Move pivot to its final place
    return storeIndex
    */
    //根据pivot位置的数,将left,right之间的数分为两部分
    fn partition(nums: &mut Vec<i32>, left: usize, right: usize, pivot: usize) -> usize {
        let pivotValue = nums[pivot];
        Solution::swap(nums, right, pivot);
        let mut store_index = left;
        for i in left..right {
            if nums[i] > pivotValue {
                //求的是第k大
                Self::swap(nums, store_index, i);
                store_index += 1;
            }
        }
        Self::swap(nums, right, store_index);
        return store_index;
    }
    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let t = nums[i];
        nums[i] = nums[j];
        nums[j] = t;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        //[3,2,3,1,2,4,5,5,6] 和 k = 4
        let t = Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);
        assert_eq!(t, 4);
    }
}
