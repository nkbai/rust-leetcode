/*
622. 设计循环队列
https://leetcode-cn.com/problems/design-circular-queue/

设计你的循环队列实现。 循环队列是一种线性数据结构，其操作表现基于 FIFO（先进先出）原则并且队尾被连接在队首之后以形成一个循环。它也被称为“环形缓冲器”。

循环队列的一个好处是我们可以利用这个队列之前用过的空间。在一个普通队列里，一旦一个队列满了，我们就不能插入下一个元素，即使在队列前面仍有空间。但是使用循环队列，我们能使用这些空间去存储新的值。

你的实现应该支持如下操作：

MyCircularQueue(k): 构造器，设置队列长度为 k 。
Front: 从队首获取元素。如果队列为空，返回 -1 。
Rear: 获取队尾元素。如果队列为空，返回 -1 。
enQueue(value): 向循环队列插入一个元素。如果成功插入则返回真。
deQueue(): 从循环队列中删除一个元素。如果成功删除则返回真。
isEmpty(): 检查循环队列是否为空。
isFull(): 检查循环队列是否已满。


示例：

MyCircularQueue circularQueue = new MycircularQueue(3); // 设置长度为 3

circularQueue.enQueue(1);  // 返回 true

circularQueue.enQueue(2);  // 返回 true

circularQueue.enQueue(3);  // 返回 true

circularQueue.enQueue(4);  // 返回 false，队列已满

circularQueue.Rear();  // 返回 3

circularQueue.isFull();  // 返回 true

circularQueue.deQueue();  // 返回 true

circularQueue.enQueue(4);  // 返回 true

circularQueue.Rear();  // 返回 4



提示：

所有的值都在 0 至 1000 的范围内；
操作数将在 1 至 1000 的范围内；
请不要使用内置的队列库。


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/design-circular-queue
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
这个题比较简单,就是一个链表, 有首有尾
不过`但是使用循环队列，我们能使用这些空间去存储新的值。`这句话不满足.
如果是较大的对象,应该考虑支持空间反复利用.如果是从这个角度,那么就不应该做成链表,
直接使用Vector最好
*/

struct MyCircularQueue {
    v: Vec<i32>,
    head: i32, //-1表示没有任何数据
    tail: i32, //-1表示没有任何数据,其他情况指向下一个可用下标
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    /** Initialize your data structure here. Set the size of the queue to be k. */
    fn new(k: i32) -> Self {
        MyCircularQueue {
            v: vec![0; k as usize],
            head: -1,
            tail: -1,
        }
    }

    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        if self.is_empty() {
            self.tail = 1;
            self.head = 0;
            self.v[0] = value;
        } else {
            self.v[self.tail as usize] = value;
            self.tail += 1;
            if self.tail >= self.v.len() as i32 {
                self.tail = 0;
            }
        }
        true
    }

    /** Delete an element from the circular queue. Return true if the operation is successful. 从前往后删 */
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head += 1;
        if self.head >= self.v.len() as i32 {
            self.head = 0;
        }
        if self.tail == self.head {
            self.tail = -1;
            self.head = -1; //没有数据了,都记为-1
        }
        true
    }

    /** Get the front item from the queue. */
    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.v[self.head as usize];
    }

    /** Get the last item from the queue. */
    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let mut l = self.tail - 1;
        if l < 0 {
            l = (self.v.len() - 1) as i32;
        }
        self.v[l as usize]
    }

    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        return self.head == -1 && self.tail == -1;
    }

    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        return self.head == self.tail && self.tail != -1;
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_design() {
        let mut obj = MyCircularQueue::new(3);
        assert_eq!(true, obj.en_queue(1));
        assert_eq!(true, obj.en_queue(2));
        assert_eq!(true, obj.en_queue(3));
        assert_eq!(false, obj.en_queue(4));
        assert_eq!(3, obj.rear());
        assert_eq!(true, obj.is_full());
        assert_eq!(true, obj.de_queue());
        assert_eq!(true, obj.en_queue(4));
        assert_eq!(4, obj.rear());

        //        ["MyCircularQueue","enQueue","enQueue","enQueue","enQueue","deQueue","deQueue","isEmpty","isEmpty","Rear","Rear","deQueue"]
        //        [[8],[3],[9],[5],[0],[],[],[],[],[],[],[]]
        let mut obj = MyCircularQueue::new(8);
        assert_eq!(true, obj.en_queue(3));
        assert_eq!(true, obj.en_queue(9));
        assert_eq!(true, obj.en_queue(5));
        assert_eq!(true, obj.en_queue(0));
        assert_eq!(true, obj.de_queue());
        assert_eq!(true, obj.de_queue());
        assert_eq!(false, obj.is_empty());
        assert_eq!(false, obj.is_empty());
        assert_eq!(0, obj.rear());
        assert_eq!(0, obj.rear());
        assert_eq!(true, obj.de_queue());
    }
}
