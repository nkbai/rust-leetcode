#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
307. 区域和检索 - 数组可修改
给定一个整数数组  nums，求出数组从索引 i 到 j  (i ≤ j) 范围内元素的总和，包含 i,  j 两点。

update(i, val) 函数可以通过将下标为 i 的数值更新为 val，从而对数列进行修改。

示例:

Given nums = [1, 3, 5]

sumRange(0, 2) -> 9
update(1, 2)
sumRange(0, 2) -> 8
说明:

数组仅可以在 update 函数下进行修改。
你可以假设 update 函数与 sumRange 函数的调用次数是均匀分布的。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/range-sum-query-mutable
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
一个简单的思路是求出[0:i]的累加和,这样求sumRange复杂度就是1,
但是Update的复杂度就是O(N),其中N是数组的长度.
显然不合适.

诉诸线段树,这样sumRange的复杂度是O(logN),update的复杂度也是O(logN)
*/
#[derive(Debug)]
struct SegmentTreeNode {
    //本节点的起始地址
    start: usize,
    end: usize,
    sum: i32,
    left: Option<Box<SegmentTreeNode>>,
    right: Option<Box<SegmentTreeNode>>,
}
impl SegmentTreeNode {
    fn new(nums: &[i32]) -> Self {
        Self::build(nums, 0, nums.len() - 1)
    }
    //组装成二叉树形状的线段树
    fn build(nums: &[i32], start: usize, end: usize) -> SegmentTreeNode {
        if start == end {
            return SegmentTreeNode {
                start,
                end,
                sum: nums[start],
                left: None,
                right: None,
            };
        }
        let mid = (start + end) / 2;
        let left = Self::build(nums, start, mid);
        let right = Self::build(nums, mid + 1, end);
        SegmentTreeNode {
            start,
            end,
            sum: left.sum + right.sum,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    /*
    比如[0,1,2,3,4]
    如果更新0,则会涉及到区间[0,0],[0,1],[0,1,2],[0,1,2,3,4]
    */
    fn update(&mut self, i: usize, val: i32) {
        if self.start == self.end && self.start == i {
            self.sum = val;
            return;
        }
        if self.start > i || self.end < i {
            panic!("range error,i={},node={:?}", i, self);
        }
        let left = self.left.as_mut().unwrap();
        let right = self.right.as_mut().unwrap();
        if i <= left.end {
            left.update(i, val);
        } else {
            right.update(i, val);
        }
        self.sum = left.sum + right.sum;
    }
    //查询一个区间和,
    //以[0-4]为例,如果想查询[1,3]
    //那么[0-4]的left为[0-2],right为[3-4]
    //所以在左侧查[1-2],右侧查[3-3]
    //以此类推,知道得到结果
    fn sum_range(&self, start: usize, end: usize) -> i32 {
        if self.start == start && self.end == end {
            return self.sum;
        }
        //不匹配,那么一定由self.start<=start<=end<=self.end
        //否则应该panic
        if !(self.start <= start && start <= end && end <= self.end) {
            panic!("range error,start={},end={},node={:?}", start, end, self);
        }
        let left = self.left.as_ref().unwrap();
        let right = self.right.as_ref().unwrap();
        if end <= left.end {
            //全部落在了左侧区域
            return left.sum_range(start, end);
        } else if right.start <= start {
            //全部落在了右侧区域
            return right.sum_range(start, end);
        } else {
            //左右多有
            let s1 = left.sum_range(start, left.end);
            let s2 = right.sum_range(right.start, end);
            return s1 + s2;
        }
    }
}
struct NumArray {
    root: Option<SegmentTreeNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        if nums.len() == 0 {
            return Self { root: None };
        }
        Self {
            root: Some(SegmentTreeNode::new(nums.as_slice())),
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        self.root.as_mut().unwrap().update(i as usize, val);
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.root
            .as_ref()
            .unwrap()
            .sum_range(i as usize, j as usize)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut t = NumArray::new(vec![1, 3, 5]);
        assert_eq!(t.sum_range(0, 2), 9);
        t.update(1, 2);
        assert_eq!(t.sum_range(0, 2), 8);
    }
}
