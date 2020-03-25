use std::collections::HashMap;

#[allow(dead_code)]
#[allow(unused_imports)]
#[macro_use]
#[allow(unused)]
#[allow(unused_variables)]
/*
952. 按公因数计算最大组件大小
给定一个由不同正整数的组成的非空数组 A，考虑下面的图：

有 A.length 个节点，按从 A[0] 到 A[A.length - 1] 标记；
只有当 A[i] 和 A[j] 共用一个大于 1 的公因数时，A[i] 和 A[j] 之间才有一条边。
返回图中最大连通组件的大小。



示例 1：

输入：[4,6,15,35]
输出：4
https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/01/ex1.png

示例 2：

输入：[20,50,9,63]
输出：2
https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/01/ex2.png

示例 3：

输入：[2,3,6,7,4,12,21,39]
输出：8
https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/01/ex3.png

提示：

1 <= A.length <= 20000
1 <= A[i] <= 100000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/largest-component-size-by-common-factor
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


 */

/*
思路:
适合用并查集,是一个分类问题.

直观的方法:
复杂度:O(N^2)
任意两个数,如果他们有公约数,那么放在一个集合中(union操作),
最糟糕的情形是任意两个数都没有公约数,这会导致N^2次公约数查找.

改进一点的方法:
求出每一个数的所有因子,然后放到一个集合里,
举例来说: 15,因子有3,5,那么15,3,5就放在一个集合里.
复杂度:O(L*Sqrt(N))
空间复杂度:O(N)
其中N是数组中最大的数字.L是数组的长度
*/
struct Solution {}
struct DSU {
    pre: Vec<i32>,
}
impl DSU {
    fn new(n: usize) -> DSU {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(i as i32);
        }
        DSU { pre: v }
    }
    fn find(&mut self, x: i32) -> i32 {
        if self.pre[x as usize] == x {
            return x;
        }
        // let prex = self.pre[x];
        let prex = self.find(self.pre[x as usize]);
        //因为递归,这里会把一串上面的所有路径都压缩了,
        self.pre[x as usize] = prex;
        return prex;
    }
    //返回false,说明x,y在同一个集合里
    fn merge(&mut self, x: i32, y: i32) -> bool {
        let prex = self.find(x);
        let prey = self.find(y);
        if prex == prey {
            return false;
        }
        //注意这里是设置的是prex的parent,而不是x的parent
        self.pre[prey as usize] = prex;
        return true;
    }
}

impl Solution {
    pub fn largest_component_size(a: Vec<i32>) -> i32 {
        let mut max = 0;
        a.iter().for_each(|n| {
            if max < *n {
                max = *n;
            }
        });
        let mut dsu = DSU::new((max + 1) as usize);
        a.iter().for_each(|n| {
            let n = *n;
            let nf = n as f32;
            // println!("n={} nf.sqrt()={}", n, nf.sqrt() as i32);
            for k in 2..=(nf.sqrt() as i32) {
                if n % k == 0 {
                    // println!("merge {}->{},{}->{}", n, k, n, n / k);
                    dsu.merge(n, k);
                    dsu.merge(n, n / k);
                    // println!("pre[{}]={},pre[{}]={}", k, n, n / k, n);
                }
            }
        });
        //这时候再逐个统计每个元素的parent,因为没有了merge的过程,得到的是最终的直接的parent.
        let mut m = HashMap::new();
        let mut max = 0;
        a.iter().for_each(|n| {
            let n = *n;
            let p = dsu.find(n);
            // println!("find {}->{}", n, p);
            let e = m.entry(p).or_insert(0);
            *e += 1;
            if max < *e {
                max = *e;
            }
        });
        max
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::largest_component_size(vec![4, 6, 15, 35]);
        assert_eq!(t, 4);
        let t = Solution::largest_component_size(vec![20, 50, 9, 63]);
        assert_eq!(t, 2);
        let t = Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39]);
        assert_eq!(t, 8);
    }
}
