use crate::utils::doubly_linked_list::DoublyLinkedList;

use super::Cache;
use std::{collections::HashMap, hash::Hash};

pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, CacheValue<V>>,
    lru_order: DoublyLinkedList<K>,
}

struct CacheValue<V> {
    value: V,
    index: usize,
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where
    K: Eq + Hash + Copy,
{
    fn new(capacity: usize) -> Self {
        let capacity = if capacity > 0 { capacity } else { 10 };
        LRUCache {
            map: HashMap::with_capacity(capacity),
            capacity,
            lru_order: DoublyLinkedList::with_capacity(capacity),
        }
    }

    fn evact(&mut self) -> Option<V> {
        let lru_key = self.lru_order.remove_bottom();
        if let Some(lru_key) = lru_key {
            if let Some(evacted_entry) = self.map.remove(&lru_key) {
                return Some(evacted_entry.value);
            }
        }
        None
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        let entry = self.map.get(key);
        if let Some(entry) = entry {
            let index = entry.index;
            self.lru_order.shift(index);
            return Some(&entry.value);
        }
        None
    }

    fn put(&mut self, key: K, value: V) -> Result<(), &'static str> {
        if self.map.len() == self.capacity {
            self.evact();
        }
        match self.lru_order.shift_new(key) {
            Ok(index) => {
                self.map.insert(key, CacheValue { value, index });
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
