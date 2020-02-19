#[allow(dead_code)]
struct MyStack {
    q: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    #[allow(dead_code)]
    fn new() -> Self {
        MyStack { q: Vec::new() }
    }

    /** Push element x onto stack. */
    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.q.push(x)
    }

    /** Removes the element on top of the stack and returns that element. */
    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        self.q.remove(self.q.len() - 1)
    }

    /** Get the top element. */
    #[allow(dead_code)]
    fn top(&self) -> i32 {
        *self.q.last().unwrap()
    }

    /** Returns whether the stack is empty. */
    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.q.len() == 0
    }
}
