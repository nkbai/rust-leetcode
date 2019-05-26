#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { qdata: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.qdata.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        if self.qdata.len() > 0 {
            Some(self.qdata.remove(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut q = Queue::new();
        q.push(1);
        q.push(2);
        println!("{:?}", q);
        assert_eq!(q.pop(), Some(1));
        println!("{:?}", q);
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), None);
        q.pop();
    }
}
