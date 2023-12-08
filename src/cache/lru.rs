use crate::utils::doubly_linked_list::DoublyLinkedList;

use super::Cache;
use std::{collections::HashMap, hash::Hash};

pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    frequencies: DoublyLinkedList<K>,
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
            frequencies: DoublyLinkedList::with_capacity(capacity),
        }
    }

    fn evact(&mut self) -> Option<V> {
        let lru_key = self.frequencies.remove_bottom();
        if let Some(lru_key) = lru_key {
            self.map.remove(&lru_key)
        } else {
            None
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn put(&mut self, key: K, value: V) -> Result<(), &'static str> {
        if self.map.len() == self.capacity {
            let key = self.frequencies.remove_bottom();
            match key {
                Some(key) => {
                    self.map.remove(&key);
                }
                _ => unreachable!(),
            }
        }
        self.map.insert(key, value);
        self.frequencies.shift(key)
    }
}
