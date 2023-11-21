use std::hash::Hash;
pub mod lru_cache;

pub trait Cache<K, V>
where
    K: Eq + Hash,
{
    /// Instantiates a new Cache
    fn new(capacity: usize) -> Self;

    /// Puts a new key-value pair into the cache
    fn put(&mut self, key: K, value: V) -> Result<(), String>;

    /// Returns the stored value against the given key
    fn get(&self, key: &K) -> Option<&V>;

    /// When the Cache capacity is filled, this function removes key-value pair
    /// based on different policies.
    fn evact(&self, key: K) -> Result<(), String>;
}
