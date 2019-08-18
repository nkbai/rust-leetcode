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
*/
use std::collections::hash_map::Entry;
use std::collections::HashMap;
#[derive(Clone, Debug)]
struct LRUCache {
    list: Vec<i32>,
    map: HashMap<i32, i32>, //value,位置
    cap: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            list: Vec::new(),
            map: HashMap::new(),
            cap: capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.entry(key) {
            Entry::Occupied(mut x) => {
                for i in 0..self.list.len() {
                    if self.list[i] == key {
                        self.list.remove(i);
                        break;
                    }
                }
                self.list.push(key);
                *x.get()
            }
            Entry::Vacant(_) => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        //存在的情形
        if self.get(key) > 0 {
            *self.map.get_mut(&key).unwrap() = value;
            return;
        }
        if self.list.len() >= self.cap as usize {
            let k = self.list.remove(0);
            self.map.remove(&k);
        }
        self.list.push(key);
        self.map.insert(key, value);
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
//
//["LRUCache","put","put","put","put","get","get","get","get","put","get","get","get","get","get"]
//[[3],[1,1],[2,2],[3,3],[4,4],[4],[3],[2],[1],[5,5],[1],[2],[3],[4],[5]]
//[null,null,null,null,null,4,3,2,-1,null,-1,2,3,-1,5]
