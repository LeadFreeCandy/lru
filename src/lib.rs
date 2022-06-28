//! A fast, 100% safe, stack based least recently used cache. 
//! 
//! fast-lru uses a stack based array to store all the values, in conjunction 
//! with a hashmap to store the keys. It gaurentees `O(1)` time complexity 
//! for all operations, including `get()`, `put()`, `get_mut()`, and `pop()`.
//! 
//! 
//! ## Example 
//! 
//! This a simple example of creating a cache, adding some values, and then reading them.
//! 
//! ```rust,no_run
//! use lru::LruCache
//! 
//! fn main() {
//!     let mut cache: LruCache<_, _, 2> = LruCache::new();
//! 
//!     cache.put("cow", 3);
//!     cache.put("pig", 2);
//! 
//!     assert_eq!(*cache.get(&"cow").unwrap(), 3);
//!     assert_eq!(*cache.get(&"pig").unwrap(), 2);
//!     assert!(cache.get(&"dog").is_none());
//! 
//!     assert_eq!(cache.put("pig", 4), Some(2));
//!     assert_eq!(cache.put("dog", 5), None);
//! 
//!     assert_eq!(*cache.get(&"dog").unwrap(), 5);
//!     assert_eq!(*cache.get(&"pig").unwrap(), 4);
//!     assert!(cache.get(&"cow").is_none());
//! 
//!     {
//!         let v = cache.get_mut(&"pig").unwrap();
//!         *v = 6;
//!     }
//! 
//!     assert_eq!(*cache.get(&"pig").unwrap(), 6);
//! }
//! ```


#![deny(unsafe_code)]
// #![warn(missing_docs)]

use std::{collections::HashMap};
use std::hash::Hash;
use arrayvec::ArrayVec;


const NULL: usize = std::usize::MAX;

#[derive(Debug)]
struct Entry<K, V>{
    prev: usize,
    key: K,
    value: V,
    next: usize,
}

pub struct LruCache<K, V, const CAP: usize> {
    // entries: HashMap<K, V>,
    // keys_to_vec: HashMap,
    // capacity: usize,
    locations: HashMap<K, usize>,
    head: usize,
    tail: usize,
    values: ArrayVec<Entry<K, V>, CAP>,
}

impl <K, V, const CAP: usize> LruCache<K, V, CAP> 
where K: Hash + Eq + Clone{

    /// Creates a new cache with the given capacity. Capacity must be specified
    /// as a constant parameter.
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use lru::LruCache;
    /// 
    /// // Create a cache with a capacity of 2.
    /// let mut cache: LruCache<_, _, 2> = LruCache::new();
    /// ```
    /// 
    pub fn new() -> Self{
        Self {
            // capacity,
            locations: HashMap::new(),
            values: ArrayVec::new(),
            head: NULL,
            tail: NULL,
        }

    }

    /// Puts the key and value into the cache. If the key already exists, it will 
    /// be moved to the front of the list and the value will be updated. If the 
    /// cache is full, the least recently used key will be removed and returned.
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use lru::LruCache;
    /// 
    /// let mut cache: LruCache<_, _, 2> = LruCache::new();
    /// 
    /// assert_eq!(cache.put("cow", 3), None);
    /// assert_eq!(cache.put("pig", 2), None);
    /// assert_eq!(cache.put("pig", 4), Some(2));
    /// ```
    /// 
    pub fn put(&mut self, key: K, value: V) -> Option<V>{
        let k2 = key.clone();
        let val = self.locations.remove(&key);

        let mut removed_value = None;

        if let Some(removed_index) = val {
            
            if self.len() == 1{
                // removed_value = Some(self.values[removed_index].value);

                let old_val = std::mem::replace(&mut self.values[0].value, value);
                removed_value = Some(old_val);
                // self.values[0].value = value;
            } else {
                let removed_entry = &mut self.values[removed_index];

                let old_val = std::mem::replace(&mut removed_entry.value, value);
                removed_value = Some(old_val);
                // removed_entry.value = value;
                self.move_to_head(removed_index);
            }

            
        } else {
            if self.values.is_empty(){
                self.head = 0;
                self.tail = 0;
                self.add_first(key, value)
            } else {
                if self.values.len() == CAP{
                    let old_index = self.remove_tail();
                    self.add_to_head_indexed(old_index, key, value)
                } else {
                    self.add_to_head(key, value);
                }
            }
        }

        self.locations.insert(k2, self.head);
        removed_value
        // println!("{:?}", self.values);
    }

    /// Puts the key and value into the cache. If the key already exists, it will 
    /// be moved to the front of the list and the value will be updated. If the 
    /// cache is full, the least recently used key will be removed and the key 
    /// value pair will be returned.
    ///
    /// # Example
    /// 
    /// ```rust,no_run
    /// use lru::LruCache;
    /// 
    /// let mut cache: LruCache<_, _, 2> = LruCache::new();
    /// 
    /// assert_eq!(cache.put_key_value("cow", 3), None);
    /// assert_eq!(cache.put_key_value("pig", 2), None);
    /// assert_eq!(cache.put_key_value("pig", 4), Some(("pig", 2)));
    /// ```
    /// 
    pub fn push(&mut self, key: K, value: V) -> Option<(K, V)>{
        let k2 = key.clone();
        let val = self.locations.remove(&key);

        let mut removed_value = None;

        if let Some(removed_index) = val {
            
            if self.len() == 1{
                // removed_value = Some(self.values[removed_index].value);

                let old_val = std::mem::replace(&mut self.values[0].value, value);
                removed_value = Some((key, old_val));
                // self.values[0].value = value;
            } else {
                let removed_entry = &mut self.values[removed_index];

                let old_val = std::mem::replace(&mut removed_entry.value, value);
                removed_value = Some((key, old_val));
                // removed_entry.value = value;
                self.move_to_head(removed_index);
            }

            
        } else {
            if self.values.is_empty(){
                self.head = 0;
                self.tail = 0;
                self.add_first(key, value)
            } else {
                if self.values.len() == CAP{
                    let old_index = self.remove_tail();
                    self.add_to_head_indexed(old_index, key, value)
                } else {
                    self.add_to_head(key, value);
                }
            }
        }

        self.locations.insert(k2, self.head);
        removed_value
        // println!("{:?}", self.values);
    }

    /// Gets a reference to the value associated with the key. If the key is not in the cache,
    /// None will be returned. If the key is in the cache, the value will be
    /// moved to the front of the list and returned. 
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use lru::LruCache;
    /// 
    /// let mut cache: LruCache<_, _, 2> = LruCache::new();
    /// 
    /// assert_eq!(cache.get("cow"), None);
    /// cache.put("cow", 3);
    /// assert_eq!(cache.get("cow"), Some(3));
    /// ```
    /// 
    pub fn get(&mut self, key: &K) -> Option<&V>{
        // let maybe_removed = self.locations.get(key);

        if let Some(index) = self.locations.get(key){
            let index = *index;

            if self.len() != 1{
                self.move_to_head(index);
            }

            Some(&self.values[index].value)
        } else{
            None
        }
    }

    /// Gets a mutable reference to the value associated with the key. If the key is not in the cache,
    /// None will be returned. If the key is in the cache, the value will be
    /// moved to the front of the list and returned.
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use lru::LruCache;
    /// 
    /// let mut cache: LruCache<_, _, 2> = LruCache::new();
    /// 
    /// assert_eq!(cache.get_mut("cow"), None);
    /// cache.put("cow", 3);
    /// assert_eq!(cache.get_mut("cow"), Some(&mut 3));
    /// ```
    /// 
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>{
        if let Some(index) = self.locations.get(key){
            let index = *index;

            if self.len() != 1{
                self.move_to_head(index);
            }

            Some(&mut self.values[index].value)
        } else{
            None
        }
    }

    /// Removes the key value pair from the cache. If the key is not in the cache,
    /// None will be returned. If the key is in the cache, the value will be
    /// removed and returned.
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use lru::LruCache;
    /// 
    /// let mut cache: LruCache<_, _, 2> = LruCache::new();
    /// 
    /// assert_eq!(cache.remove("cow"), None);
    /// cache.put("cow", 3);
    /// assert_eq!(cache.remove("cow"), Some(3));
    /// assert_eq!(cache.get("cow"), None);
    /// ```
    /// 
    pub fn pop(&mut self, key: &K) -> Option<V>{
        if let Some(index) = self.locations.remove(key){
            self.move_to_tail(index);
            Some(self.values.pop().unwrap().value)
        } else {
            None
        }
    }
    
    /// Returns the number of key values that can be stored in the cache.
    pub fn capacity(&self) -> usize{
        CAP
    }

    /// Clears the cache.
    pub fn clear(&mut self){
        self.locations.clear();
        self.values.clear();
        self.head = NULL;
        self.tail = NULL;
    }

    fn move_to_head(&mut self, index: usize){

        let entry = &mut self.values[index];
        let prev = entry.prev;
        let next = entry.next;

        if prev == NULL && next == NULL {
            todo!();
        }

        if prev != NULL {
            if next != NULL{
                self.values[prev].next = next;
            } else {
                self.values[prev].next = NULL;
            }
        } else {
            self.head = next;
        }

        if next != NULL {
            if prev != NULL {
                self.values[next].prev = prev;
            } else {
                self.values[next].prev = NULL;
            }
        } else {
            self.tail = prev;
        }

        let entry = &mut self.values[index];

        let head_index = self.head;
        entry.prev = NULL;
        entry.next = head_index;

        self.head = index;
        self.values[head_index].prev = self.head;

    }

    fn swap_with_last(&mut self, index: usize){
        let last_index = self.values.len() - 1;

        let last_entry = &mut self.values[last_index];
        let last_prev = last_entry.prev;
        let last_next = last_entry.next;

        if last_prev != NULL{
            self.values[last_prev].next = index;
        } else {
            self.head = index;
        }

        if last_next != NULL {
            self.values[last_next].prev = index;
        } else {
            self.tail = index;
        }

        self.values.swap(last_index, index);
    }

    fn move_to_tail(&mut self, index: usize){
        let entry = &mut self.values[index];
        let prev = entry.prev;
        let next = entry.next;

        if prev != NULL {
            if next != NULL{
                self.values[prev].next = next;
            } else {
                self.values[prev].next = NULL;
            }
        } else {
            self.head = next;
        }

        if next != NULL {
            if prev != NULL {
                self.values[next].prev = prev;
            } else {
                self.values[next].prev = NULL;
            }
        } else {
            self.tail = prev;
        }

        self.swap_with_last(index);
    }

    fn add_to_head(&mut self, key: K, value: V){
        let head_index = self.head;

        let new_entry = Entry {
            prev: NULL,
            key,
            value,
            next: head_index,
        };

        self.values.push(new_entry);

        self.head = self.values.len() - 1;

        self.values[head_index].prev = self.head;
    }

    fn add_to_head_indexed(&mut self, index: usize, key: K, value: V){
        let head_index = self.head;

        let new_entry = Entry {
            prev: NULL,
            key,
            value,
            next: head_index,
        };

        self.values[index] = new_entry;

        self.head = index;

        self.values[head_index].prev = self.head;
    }

    fn remove_tail(&mut self) -> usize{

        let old_tail_index= self.tail;
        let tail_entry = &mut self.values[old_tail_index];
        self.locations.remove(&tail_entry.key);

        let new_tail_index = tail_entry.prev;
        let new_tail_entry = &mut self.values[new_tail_index];

        new_tail_entry.next = NULL;
        self.tail = new_tail_index;

        old_tail_index
    }

    fn add_first(&mut self, key: K, value: V){
        let new_entry = Entry {
            prev: NULL,
            key,
            value,
            next: NULL,
        };

        self.values.push(new_entry);

    }

    /// Returns the number of key values in the cache.
    pub fn len(&self) -> usize{
        self.values.len()
    }

    pub fn is_empty(&self) -> bool{
        self.len() == 0
    }
}



#[cfg(test)]
mod tests {

    use super::LruCache;

    fn assert_opt_eq<V: PartialEq>(opt: Option<&V>, v: V) {
        assert!(opt.is_some());
        assert!(opt.unwrap() == &v);
    }

    #[test]
    fn test_put_and_get() {
        let mut cache: LruCache<_, _, 2> = LruCache::new();
        cache.put(1, 10);
        cache.put(2, 20);
        assert_opt_eq(cache.get(&1), 10);
        assert_opt_eq(cache.get(&2), 20);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_put_update() {
        let mut cache: LruCache<String, Vec<u8>, 1> = LruCache::new();
        cache.put("1".to_string(), vec![10, 10]);
        cache.put("1".to_string(), vec![10, 19]);
        assert_opt_eq(cache.get(&"1".to_string()), vec![10, 19]);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_expire_lru() {
        let mut cache: LruCache<String, String, 2> = LruCache::new();
        cache.put("foo1".to_string(), "bar1".to_string());
        cache.put("foo2".to_string(), "bar2".to_string());
        cache.put("foo3".to_string(), "bar3".to_string());
        assert!(cache.get(&"foo1".to_string()).is_none());
        cache.put("foo2".to_string(), "bar2update".to_string());
        cache.put("foo4".to_string(), "bar4".to_string());
        assert!(cache.get(&"foo3".to_string()).is_none());
    }

    #[test]
    fn test_pop() {
        let mut cache: LruCache<_,_,2> = LruCache::new();
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.len(), 2);
        let opt1 = cache.pop(&1);
        assert!(opt1.is_some());
        assert_eq!(opt1.unwrap(), 10);
        assert!(cache.get(&1).is_none());
        assert_eq!(cache.len(), 1);
    }


    #[test]
    fn test_clear() {
        let mut cache: LruCache<_, _, 2> = LruCache::new();
        cache.put(1, 10);
        cache.put(2, 20);
        cache.clear();
        assert!(cache.get(&1).is_none());
        assert!(cache.get(&2).is_none());
        assert_eq!(cache.len(), 0);
        // assert_eq!(cache.to_string(), "{}".to_string());
    }

    #[test]
    fn example() {
        let mut cache: LruCache<_, _, 2> = LruCache::new();

        cache.put("cow", 3);
        cache.put("pig", 2);

        assert_eq!(*cache.get(&"cow").unwrap(), 3);
        assert_eq!(*cache.get(&"pig").unwrap(), 2);
        assert!(cache.get(&"dog").is_none());

        assert_eq!(cache.put("pig", 4), Some(2));
        assert_eq!(cache.put("dog", 5), None);

        assert_eq!(*cache.get(&"dog").unwrap(), 5);
        assert_eq!(*cache.get(&"pig").unwrap(), 4);
        assert!(cache.get(&"cow").is_none());

        {
            let v = cache.get_mut(&"pig").unwrap();
            *v = 6;
        }

        assert_eq!(*cache.get(&"pig").unwrap(), 6);
    }
}