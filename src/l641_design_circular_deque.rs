/*
https://leetcode-cn.com/problems/design-circular-deque/
641. 设计循环双端队列

设计实现双端队列。
你的实现需要支持以下操作：

MyCircularDeque(k)：构造函数,双端队列的大小为k。
insertFront()：将一个元素添加到双端队列头部。 如果操作成功返回 true。
insertLast()：将一个元素添加到双端队列尾部。如果操作成功返回 true。
deleteFront()：从双端队列头部删除一个元素。 如果操作成功返回 true。
deleteLast()：从双端队列尾部删除一个元素。如果操作成功返回 true。
getFront()：从双端队列头部获得一个元素。如果双端队列为空，返回 -1。
getRear()：获得双端队列的最后一个元素。 如果双端队列为空，返回 -1。
isEmpty()：检查双端队列是否为空。
isFull()：检查双端队列是否满了。
示例：
```
MyCircularDeque circularDeque = new MycircularDeque(3); // 设置容量大小为3
circularDeque.insertLast(1);			        // 返回 true
circularDeque.insertLast(2);			        // 返回 true
circularDeque.insertFront(3);			        // 返回 true
circularDeque.insertFront(4);			        // 已经满了，返回 false
circularDeque.getRear();  				// 返回 2
circularDeque.isFull();				        // 返回 true
circularDeque.deleteLast();			        // 返回 true
circularDeque.insertFront(4);			        // 返回 true
circularDeque.getFront();				// 返回 4
```


提示：

所有值的范围为 [1, 1000]
操作次数的范围为 [1, 1000]
请不要使用内置的双端队列库。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/design-circular-deque
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
思路:
用Vec来实现即可
*/

struct MyCircularDeque {
    v: Vec<i32>,
    k: usize, //最大容量
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    /** Initialize your data structure here. Set the size of the deque to be k. */
    fn new(k: i32) -> Self {
        let k = k as usize;
        let m = MyCircularDeque { v: Vec::new(), k };
        return m;
    }
    /** Adds an item at the front of Deque. Return true if the operation is successful. */
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.v.insert(0, value);
        true
    }

    /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.v.push(value);
        true
    }

    /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.v.remove(0);
        true
    }

    /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.v.remove(self.v.len() - 1);
        true
    }

    /** Get the front item from the deque. */
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        return self.v[0];
    }

    /** Get the last item from the deque. */
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        return self.v[self.v.len() - 1];
    }

    /** Checks whether the circular deque is empty or not. */
    fn is_empty(&self) -> bool {
        return self.v.len() == 0;
    }

    /** Checks whether the circular deque is full or not. */
    fn is_full(&self) -> bool {
        return self.v.len() == self.k;
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mycircular_deque() {
        let mut obj = MyCircularDeque::new(3);
        assert_eq!(true, obj.insert_front(3));
        assert_eq!(true, obj.insert_last(2));
        assert_eq!(true, obj.delete_front());
        assert_eq!(true, obj.delete_last());

        let mut obj = MyCircularDeque::new(3);
        obj.insert_front(3);
        obj.insert_front(2);
        obj.insert_front(1);
        assert_eq!(true, obj.is_full());
        assert_eq!(3, obj.get_rear());
        assert_eq!(1, obj.get_front());
    }
}
