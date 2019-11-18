/*
155. 最小栈

设计一个支持 push，pop，top 操作，并能在常数时间内检索到最小元素的栈。

push(x) -- 将元素 x 推入栈中。
pop() -- 删除栈顶的元素。
top() -- 获取栈顶元素。
getMin() -- 检索栈中的最小元素。
示例:

MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/min-stack
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
一开始选择用BTreeMap,但是没有考虑到重复的问题,
如果元素是有重复的,那么就必须使用BTreeMap进行计数才行
*/
use std::collections::btree_map::BTreeMap;
struct MinStack {
    h: Vec<i32>,
    t: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            h: Vec::new(),
            t: BTreeMap::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.h.push(x);
        *self.t.entry(x).or_insert(0) += 1
    }

    fn pop(&mut self) {
        let x = self.h.pop().expect("must have at least one element");
        let e = self.t.entry(x);
        let mut to_delete = false;
        e.and_modify(|n| {
            if *n > 1 {
                *n -= 1;
            } else {
                to_delete = true;
            }
        });
        if to_delete {
            self.t.remove(&x);
        }
    }

    fn top(&self) -> i32 {
        *self.h.last().expect("")
    }

    fn get_min(&self) -> i32 {
        let (v, _) = self.t.iter().next().expect("must have one element");
        *v
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mininum_total() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        println!("t={:?}", s.t);
        assert_eq!(s.get_min(), -2);

        let mut s = MinStack::new();
        s.push(2);
        s.push(3);
        s.push(3);
        s.push(2);
        assert_eq!(s.get_min(), 2);
        s.pop();
        assert_eq!(s.top(), 3);
        println!("t={:?}", s.t);
        assert_eq!(s.get_min(), 2);
    }
}
