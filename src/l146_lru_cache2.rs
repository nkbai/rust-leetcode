/*

146. LRU缓存机制
运用你所掌握的数据结构，设计和实现一个  LRU (最近最少使用) 缓存机制。它应该支持以下操作： 获取数据 get 和 写入数据 put 。

获取数据 get(key) - 如果密钥 (key) 存在于缓存中，则获取密钥的值（总是正数），否则返回 -1。
写入数据 put(key, value) - 如果密钥不存在，则写入其数据值。当缓存容量达到上限时，它应该在写入新数据之前删除最近最少使用的数据值，从而为新的数据值留出空间。

进阶:

你是否可以在 O(1) 时间复杂度内完成这两种操作？

示例:
```
LRUCache cache = new LRUCache( 2 /* 缓存容量 */ );

cache.put(1, 1);
cache.put(2, 2);
cache.get(1);       // 返回  1
cache.put(3, 3);    // 该操作会使得密钥 2 作废
cache.get(2);       // 返回 -1 (未找到)
cache.put(4, 4);    // 该操作会使得密钥 1 作废
cache.get(1);       // 返回 -1 (未找到)
cache.get(3);       // 返回  3
cache.get(4);       // 返回  4
```
来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/lru-cache
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
/*
O(1)复杂度的思路:
1. 使用map来保存数据 key到映射value以及双向链表中的元素
2. get 查询,如果查询得到,从双向链表中移除并添加到尾部
3. put 如果满了,则移除双向链表首部元素,同时插入map,添加到尾部
手工模拟双向链表.
双向链表操作直接借鉴自标准库
*/
use std::collections::hash_map::Entry;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct LRUCache {
    capacity: usize,
    head: Option<i32>,
    tail: Option<i32>,
    caches: HashMap<i32, Cache>,
}

#[derive(Clone, Debug)]
struct Cache {
    value: i32,
    next: Option<i32>,
    prev: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            head: None,
            tail: None,
            caches: HashMap::new(),
        }
    }
    /*
    push_back_node,pop_front_node,unlink_node 直接借鉴自标准库中的LinkedList

    */
    fn push_back_node(&mut self, key: i32, value: i32) {
        let node = Cache {
            next: None,
            prev: self.tail,
            value,
        };
        match self.tail {
            None => self.head = Some(key),
            Some(tail) => self.caches.get_mut(&tail).unwrap().next = Some(key),
        }
        self.tail = Some(key);
        self.caches.insert(key, node);
    }
    fn pop_front_node(&mut self) {
        self.head.map(|key| {
            let c = self.caches.get(&key);
            self.head = c.unwrap().next;
            match self.head {
                None => {
                    self.tail = None;
                }
                Some(head) => self.caches.get_mut(&head).unwrap().prev = None,
            }
            self.caches.remove(&key);
            key
        });
    }
    fn unlink_node(&mut self, key: i32, c: &Cache) {
        match c.prev {
            Some(prev) => {
                let prev_cache = self.caches.get_mut(&prev).unwrap();
                prev_cache.next = c.next;
            }
            None => self.head = c.next,
        };
        match c.next {
            Some(next) => {
                let next_cache = self.caches.get_mut(&next).unwrap();
                next_cache.prev = c.prev;
            }
            None => self.tail = c.prev,
        };
        self.caches.remove(&key);
    }
    pub fn get(&mut self, key: i32) -> i32 {
        match self.caches.entry(key) {
            Entry::Occupied(x) => {
                /*
                已经存在的情况下,先移除旧的,然后添加新的到尾部
                */
                //移除旧的
                let c = x.get().clone(); //否则会出现连续两次mut借用的情况,第一次发生在Occupied,第二次发生在unlink_node
                self.unlink_node(key, &c);
                self.push_back_node(key, c.value);
                c.value
            }
            Entry::Vacant(_) => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        //存在的情形
        if self.get(key) > 0 {
            self.caches.get_mut(&key).unwrap().value = value;
            return;
        }
        if self.caches.len() >= self.capacity as usize {
            self.pop_front_node();
        }
        self.push_back_node(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lru() {
        let mut cache = LRUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1); // 返回  1
        cache.put(3, 3); // 该操作会使得密钥 2 作废
        assert_eq!(cache.get(2), -1); // 返回 -1 (未找到)
        cache.put(4, 4); // 该操作会使得密钥 1 作废
        assert_eq!(cache.get(1), -1); // 返回 -1 (未找到)
        assert_eq!(cache.get(3), 3); // 返回  3
        assert_eq!(cache.get(4), 4); // 返回  4
    }
    #[test]
    fn test_lru2() {
        let mut cache = LRUCache::new(3);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);
        assert_eq!(cache.get(4), 4);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(2), 2);
        println!("cache={:?}", cache);
        assert_eq!(cache.get(1), -1);
        cache.put(5, 5); // 该操作会使得密钥 1 作废
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), -1);
        assert_eq!(cache.get(5), 5);
    }
}
