/*
已有方法 rand7 可生成 1 到 7 范围内的均匀随机整数，试写一个方法 rand10 生成 1 到 10 范围内的均匀随机整数。

不要使用系统的 Math.random() 方法。



示例 1:

输入: 1
输出: [7]
示例 2:

输入: 2
输出: [8,4]
示例 3:

输入: 3
输出: [8,1,10]


提示:

rand7 已定义。
传入参数: n 表示 rand10 的调用次数。


进阶:

rand7()调用次数的 期望值 是多少 ?
你能否尽量少调用 rand7() ?
*/
use rand::random;

struct Solution {}
fn rand7() -> i32 {
    let r: i32 = random();
    r % 7 + 1
}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        0
    }
}
