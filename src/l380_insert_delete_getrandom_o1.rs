/*
https://leetcode-cn.com/problems/insert-delete-getrandom-o1/
设计一个支持在平均 时间复杂度 O(1) 下，执行以下操作的数据结构。

insert(val)：当元素 val 不存在时，向集合中插入该项。
remove(val)：元素 val 存在时，从集合中移除该项。
getRandom：随机返回现有集合中的一项。每个元素应该有相同的概率被返回。
示例 :

// 初始化一个空的集合。
RandomizedSet randomSet = new RandomizedSet();

// 向集合中插入 1 。返回 true 表示 1 被成功地插入。
randomSet.insert(1);

// 返回 false ，表示集合中不存在 2 。
randomSet.remove(2);

// 向集合中插入 2 。返回 true 。集合现在包含 [1,2] 。
randomSet.insert(2);

// getRandom 应随机返回 1 或 2 。
randomSet.getRandom();

// 从集合中移除 1 ，返回 true 。集合现在包含 [2] 。
randomSet.remove(1);

// 2 已在集合中，所以返回 false 。
randomSet.insert(2);

// 由于 2 是集合中唯一的数字，getRandom 总是返回 2 。
randomSet.getRandom();

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/insert-delete-getrandom-o1
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

/*
用hashmap
插入是常数,删除也是常数,
随机获取怎么处理呢?

思路：1.用slice存值，用map保存值在slice中的index；
2.每次删除时，为了避免移动元素，用数组末尾元素覆盖需要删除的元素，然后删除数组末尾元素；
*/
extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
struct RandomizedSet {
    m: HashMap<i32, usize>,
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedSet {
            m: HashMap::new(),
            v: Vec::new(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    pub fn insert(&mut self, val: i32) -> bool {
        if self.m.contains_key(&val) {
            return false;
        }
        let i = self.m.len();
        if self.v.len() > i {
            //经过删除以后v里面空间可能非常富裕,直接用现有的才对
            self.v[i] = val;
        } else {
            self.v.push(val);
        }
        self.m.insert(val, i);
        return true;
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    pub fn remove(&mut self, val: i32) -> bool {
        if !self.m.contains_key(&val) {
            return false;
        }
        let i = *self.m.get(&val).expect("must ok");
        if i != self.m.len() - 1 {
            //如果是最后一个,就不用调整了
            self.v[i] = self.v[self.m.len() - 1]; //最后一个值填充到i
            self.m.insert(self.v[i], i);
        }
        self.m.remove(&val);
        return true;
    }

    /** Get a random element from the set. */
    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut i: usize = rng.gen();
        i = i % self.m.len();
        return self.v[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_RandomizedSet() {
        let mut s = RandomizedSet::new();
        assert_eq!(true, s.insert(1));
        assert_eq!(false, s.remove(2));
        assert_eq!(true, s.insert(2));
        println!("s={}", s.get_random());
        assert_eq!(true, s.remove(1));
        assert_eq!(false, s.insert(2));
        assert_eq!(2, s.get_random());
    }
    #[test]
    fn test_RandomizedSet_zero() {
        let mut s = RandomizedSet::new();
        assert_eq!(false, s.remove(0));
        assert_eq!(true, s.insert(0));
        assert_eq!(0, s.get_random());
        assert_eq!(true, s.remove(0));
        assert_eq!(true, s.insert(0));
    }
}
