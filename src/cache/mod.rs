use std::hash::Hash;
pub mod lru;

pub trait Cache<K, V>
where
    K: Eq + Hash,
{
    /// Instantiates a new Cache
    fn new(capacity: usize) -> Self;

    /// Puts a new key-value pair into the cache
    fn put(&mut self, key: K, value: V) -> Result<(), &'static str>;

    /// Returns the stored value against the given key
    fn get(&mut self, key: &K) -> Option<&V>;

    /// When the Cache capacity is filled, this function removes key-value pair
    /// based on different policies.
    fn evact(&mut self) -> Option<V>;
}
