#![feature(extract_if)]
use std::collections::{HashMap, LinkedList};

struct LRUCache {
    item_usage: LinkedList<i32>,
    item_cache: HashMap<i32, i32>,
    capacity: i32,
}

// `&self` means the method takes an immutable reference
// If you need a mutable reference, change it to `&mut self` instead.
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            item_usage: LinkedList::new(),
            item_cache: HashMap::with_capacity((capacity + 1) as usize),
            capacity,
        }
    }

    // If we look for a value and it exists then we need to move it to the top of the LinkedList
    // from wherever it is currently.
    fn get(&mut self, key: i32) -> i32 {
        let possible_value = self.item_cache.get(&key);
        return match possible_value {
            Some(value) => {
                let my_key = key.clone();
                let _ = self.item_usage.extract_if(| x | x == &my_key);
                self.item_usage.push_front(my_key);
                // self.move_to_top(my_key);
                value.clone()
            },
            _ => { 0 }
        }
    }

    fn add_to_cache(&mut self, key: i32, value: i32) {
        self.item_cache.insert(key, value);
        self.item_usage.push_front(key);

        // if the cache is over capacity then remove the oldest entry
        if self.item_cache.len() > self.capacity as usize {
            // remove the oldest entry off the stack
            let oldest_key = self.item_usage.pop_back();
            match oldest_key {
                Some(key) => {
                    self.item_cache.remove_entry(&key);
                }
                None => {
                    println!("key not found for {}", key);
                }
            }
        }
    }

    /**
     * When adding an item to the cache we also need to add it to the usage stack
     * If we're adding an item that already exists then we need to bring it to the top of the stack
     */
    fn put(&mut self, key: i32, value: i32) {
        let possible_value = &self.item_cache.get(&key);
        match possible_value {
            Some(_some_value) => {
                // item already exists so move it to the top of the stack before returning the value
                let _ = self.item_usage.extract_if(|usage_key| usage_key == &key );
                self.item_usage.push_front(key);
            }
            None => {
                // doesn't exist in the cache so add it to the cache
                self.add_to_cache(key, value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LRUCache;

    #[test]
    fn part_a_test() {
        let mut lru_cache = LRUCache::new(3);
        println!("part 1");
        lru_cache.put(1, 1);
        println!("part 2");
        lru_cache.put(2, 2);
        println!("part 3");
        assert_eq!(lru_cache.get(1), 1);
        println!("part 4");
        assert_eq!(lru_cache.get(2), 2);
        println!("part 5");
        lru_cache.put(3, 3);
        println!("part 6");
        lru_cache.put(4, 4);
        println!("part 7");
        assert_eq!(lru_cache.get(3), 3);
        println!("part 8");
        assert_eq!(lru_cache.get(4), 4);
        println!("part 9");
        assert_eq!(lru_cache.get(1), 0);
        println!("part 10");
    }
}
