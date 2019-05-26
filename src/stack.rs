#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Clone, Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode {
            val: val,
            next: None,
        }
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { top: None }
    }

    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        #[derive(PartialEq, Eq, Debug)]
        struct TestStruct {
            a: i32,
        }

        let a = TestStruct { a: 5 };
        let b = TestStruct { a: 9 };

        let mut s = Stack::<&TestStruct>::new();
        assert_eq!(s.pop(), None);

        s.push(&a);
        s.push(&b);
        println!("{:?}", s);

        assert_eq!(s.pop(), Some(&b));
        assert_eq!(s.pop(), Some(&a));
        assert_eq!(s.pop(), None);
    }
}
