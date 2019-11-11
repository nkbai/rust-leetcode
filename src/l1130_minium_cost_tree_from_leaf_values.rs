/*
1130. 叶值的最小代价生成树

给你一个正整数数组 arr，考虑所有满足以下条件的二叉树：

每个节点都有 0 个或是 2 个子节点。
数组 arr 中的值与树的中序遍历中每个叶节点的值一一对应。（知识回顾：如果一个节点有 0 个子节点，那么该节点为叶节点。）
每个非叶节点的值等于其左子树和右子树中叶节点的最大值的乘积。
在所有这样的二叉树中，返回每个非叶节点的值的最小可能总和。这个和的值是一个 32 位整数。



示例：

输入：arr = [6,2,4]
输出：32
解释：
有两种可能的树，第一种的非叶节点的总和为 36，第二种非叶节点的总和为 32。

    24            24
   /  \          /  \
  12   4        6    8
 /  \               / \
6    2             2   4


提示：

2 <= arr.length <= 40
1 <= arr[i] <= 15
答案保证是一个 32 位带符号整数，即小于 2^31。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-cost-tree-from-leaf-values
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
要让大的数在树中的depth尽可能的小,这样总和中与他相乘的次数也就会少
采用单调递减栈来实现.
单调栈性质：

1、若是单调递增栈，则从栈顶到栈底的元素是严格递增的。若是单调递减栈，则从栈顶到栈底的元素是严格递减的。

2、越靠近栈顶的元素越后进栈。（显而易见）

因为题目中说的是数组中的数都是dfs的叶节点,那么意味着数组中相邻的两个数在树中也是相邻的,这是前提.
时间复杂度是O(N)空间复杂度也是O(N)
*/

struct Solution;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        /*
        使用单调递减栈,如果碰到逆序,就弹出,作为树的两个节点,因为如果不是他们两个相乘,
        那么就会造成左边大的数乘两次,而不是待入栈的这个数乘两次,显然不合理.
        比如 [6,2,4] ,碰到4的时候,则是弹出2,然后2,4组成一个节点
        如果是[6,2,9],碰到9的时候则是6,2都弹出,6,2组成一颗子树,然后9,6入栈
        */
        let mut stack = Vec::new();
        let mut result = 0;
        let mut arr = arr;
        stack.push(arr.remove(0)); //arr中至少包含了两个数
        while arr.len() > 0 {
            let c = arr.remove(0);
            if *stack.last().expect("must have one") >= c {
                stack.push(c);
                continue; //满足单调递减栈
            }
            let mut stack2 = Vec::new();

            while let Some(c2) = stack.last() {
                if *c2 < c {
                    //stack不满足递减栈规则,弹出到stack2,stack2是一个单调递增栈
                    stack2.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            //所有的小于c的自行结算
            while stack2.len() >= 2 {
                let top = stack2.remove(0);
                let top2 = *stack2.first().expect("must have first");
                println!("result={},top={},top2={}", result, top, top2);
                result += top * top2;
            }
            //c要和弹出部分最大的计算
            result += c * *stack2.first().expect("must have at least one number");
            println!(
                "result={},top={},top2={}",
                result,
                c,
                stack2.first().unwrap()
            );
            stack.push(c);
        }
        while stack.len() >= 2 {
            let top = stack.pop().expect("must have 2 numbers");
            let top2 = *stack.last().expect("must have last");
            println!("result={},top={},top2={}", result, top, top2);
            result += top * top2;
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::share::*;
    #[test]
    fn test() {
        assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
        assert_eq!(
            Solution::mct_from_leaf_values(vec![1, 2, 3, 4, 5]),
            2 + 6 + 12 + 20
        );
        assert_eq!(
            Solution::mct_from_leaf_values(vec![5, 4, 3, 2, 1]),
            2 + 6 + 12 + 20
        );
        assert_eq!(
            Solution::mct_from_leaf_values(vec![15, 13, 5, 3, 15]),
            3 * 5 + 13 * 5 + 15 * 13 + 15 * 15
        );
    }
}
