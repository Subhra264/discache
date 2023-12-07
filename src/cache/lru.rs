use super::Cache;
use std::{collections::HashMap, hash::Hash};

pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where
    K: Eq + Hash,
{
    fn new(capacity: usize) -> Self {
        LRUCache {
            map: HashMap::with_capacity(capacity),
            capacity,
        }
    }

    fn evact(&self, _key: K) -> Result<(), String> {
        // TODO
        Ok(())
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn put(&mut self, key: K, value: V) -> Result<(), String> {
        if self.map.len() == self.capacity {
            // TODO
            Err("Capacity full!".to_string())
        } else {
            self.map.insert(key, value);
            Ok(())
        }
    }
}
