//该版本复制自其它人的答案,用时28毫秒,可以节省get时候不必要的remove
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct LRUCache {
    capacity: usize,
    size: usize,
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
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            size: 0,
            head: None,
            tail: None,
            caches: HashMap::with_capacity(capacity as usize),
        }
    }
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.detach(key, false) {
            self.attach(key);
            return value;
        }
        -1
    }
    pub fn put(&mut self, key: i32, value: i32) {
        use std::collections::hash_map::Entry;
        let is_new = {
            let entry = self.caches.entry(key);
            match entry {
                Entry::Occupied(mut val) => {
                    let mut cache = val.get_mut();
                    cache.value = value;
                    false
                }
                Entry::Vacant(val) => {
                    let _ = val.insert(Cache {
                        value,
                        prev: None,
                        next: None,
                    });
                    true
                }
            }
        };
        if is_new {
            if self.size == self.capacity {
                if let Some(tail) = self.tail {
                    self.remove(tail);
                }
            }
            self.size += 1;
        } else {
            self.detach(key, false);
        }
        self.attach(key);
    }
    fn remove(&mut self, key: i32) -> bool {
        self.detach(key, true).is_some()
    }
    fn detach(&mut self, key: i32, remove: bool) -> Option<i32> {
        // node.prev.next = node.next
        // node.next.prev = node.prev
        let cache = if remove {
            let cache = self.caches.remove(&key);
            self.size -= 1;
            cache.map(|cache| (cache.value, cache.prev, cache.next))
        } else {
            let cache = self.caches.get(&key);
            cache.map(|cache| (cache.value, cache.prev, cache.next))
        };
        if let Some((value, prev, next)) = cache {
            if let Some(pr) = prev {
                let _ = self.caches.get_mut(&pr).map(|pr| {
                    pr.next = next;
                });
            }
            if let Some(ne) = next {
                let _ = self.caches.get_mut(&ne).map(|ne| {
                    ne.prev = prev;
                });
            }
            if self.head == Some(key) {
                self.head = prev;
            }
            if self.tail == Some(key) {
                self.tail = next;
            }
            Some(value)
        } else {
            None
        }
    }
    fn attach(&mut self, key: i32) {
        // node.next = null
        // node.prev = head
        // head.next = node
        // head = node
        if let Some(head) = self.head {
            let _ = self.caches.get_mut(&key).map(|c| {
                c.next = None;
                c.prev = Some(head);
            });
            let _ = self.caches.get_mut(&head).map(|c| {
                c.next = Some(key);
            });
        }
        self.head = Some(key);
        if self.tail.is_none() {
            self.tail = Some(key);
        }
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
