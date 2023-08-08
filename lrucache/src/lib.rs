use std::collections::LinkedList;

struct LRUCache {
    cache: LinkedList<i32>,
}


/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        cache = LinkedList::new();
    }

    fn get(&self, key: i32) -> i32 {

    }

    fn put(&self, key: i32, value: i32) {

    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    use crate::LRUCache;

    #[test]
    fn part_a_test() {
        let lruCache = LRUCache::new(3);
        assert_eq!(lruCache.put(1, 1), 1);
        assert_eq!(lruCache.put(2, 2), 2);
        assert_eq!(lruCache.get(1), 1);
        assert_eq!(lruCache.get(2), 2);
        assert_eq!(lruCache.put(3, 3), 3);
        assert_eq!(lruCache.put(4, 4), 4);
    }

}

