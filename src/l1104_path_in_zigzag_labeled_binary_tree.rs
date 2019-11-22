/*

1104. 二叉树寻路

在一棵无限的二叉树上，每个节点都有两个子节点，树中的节点 逐行 依次按 “之” 字形进行标记。

如下图所示，在奇数行（即，第一行、第三行、第五行……）中，按从左到右的顺序进行标记；

而偶数行（即，第二行、第四行、第六行……）中，按从右到左的顺序进行标记。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/06/28/tree.png)


给你树上某一个节点的标号 label，请你返回从根节点到该标号为 label 节点的路径，该路径是由途经的节点标号所组成的。



示例 1：

输入：label = 14
输出：[1,3,4,14]
示例 2：

输入：label = 26
输出：[1,2,6,10,26]


提示：

1 <= label <= 10^6

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/path-in-zigzag-labelled-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
思路:
如果不考虑zigzag,那么label就是位置,编号从1开始,假设label是26,其对应的位置就是16,
那么此时返回的应该是[14,7,3,1]
因为zigzag的缘故,要是别出来偶数行是从右到左的,其中14,3,是偶数行
14 需要从[8..15]进行转换,所以14对应的位置是9,但是因为14是第一个数,所以直接从这个位置开始.
那么9这个位置对应的父节点就是9/2=4,奇数行不做处理.
得到[14,4]
然后4的父节点是2,其所在行是[2,3],偶数行
所以转换为3,得到[14,4,3]
3的父节点为1,奇数行不转换,得到[14,4,3,1],逆序输出即可.
注意计算父节点的时候都应该使用位置,而不应该使用位置上的值
*/
struct Solution;

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut v = Vec::new();
        let mut isFirst = true;
        let mut cur_pos = label;
        while cur_pos > 0 {
            let first_one_pos = 32 - cur_pos.leading_zeros(); // leading_zero(cur_pos as u32);
            println!("cur={},first_one_pos={}", cur_pos, first_one_pos);
            let isEven = first_one_pos % 2 == 0; //第一个1是奇数行,从1开始
            let mut order_num = cur_pos;
            if isEven {
                //偶数行需要转换
                let first = 1 << (first_one_pos - 1); //针对8-15,范围是1000到1111,first_one_pos=4
                let end = (1 << first_one_pos) - 1;
                order_num = end - (order_num - first);
            }
            if isFirst {
                v.insert(0, cur_pos);
                isFirst = false;
                cur_pos = order_num; //第一个数是逆序,需要保证真正的位置
            } else {
                v.insert(0, order_num); //除第一个值以外都应该使用对应位置上的值,而不是位置本身
            }
            println!("orderPos={}", order_num);
            cur_pos = cur_pos / 2; //找下一个
        }
        return v;
    }
}
//判断一个整数有多少个前缀0
fn leading_zero(x: u32) -> i32 {
    let mut n = 0;
    let mut x = x;
    if x <= 0x0000ffff {
        n += 16;
        x <<= 16;
    }
    if x <= 0x00ffffff {
        n += 8;
        x <<= 8;
    }
    if x <= 0x0fffffff {
        n += 4;
        x <<= 4;
    }
    if x <= 0x3fffffff {
        n += 2;
        x <<= 2;
    }
    if x <= 0x7fffffff {
        n += 1;
    }
    return n;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::path_in_zig_zag_tree(14), vec![1, 3, 4, 14]);
        assert_eq!(Solution::path_in_zig_zag_tree(8), vec![1, 2, 7, 8]);
        assert_eq!(Solution::path_in_zig_zag_tree(26), vec![1, 2, 6, 10, 26]);
        assert_eq!(Solution::path_in_zig_zag_tree(16), vec![1, 3, 4, 15, 16]);
        assert_eq!(Solution::path_in_zig_zag_tree(31), vec![1, 2, 7, 8, 31]);
    }
    #[test]
    fn test_leading_zeros() {
        assert_eq!(31, leading_zero(1));
        assert_eq!(30, leading_zero(2));
        assert_eq!(28, leading_zero(14));
    }
}
