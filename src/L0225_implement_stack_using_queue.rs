struct MyStack {
 q: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack{q:Vec::new()}
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.q.push(x)
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.q.remove(self.q.len()-1)
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.q.last().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.q.len()==0
    }
}

