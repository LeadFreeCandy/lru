use std::{collections::HashMap};
use std::hash::Hash;


const NULL: usize = std::usize::MAX;

#[derive(Debug)]
struct Entry<K, V>{
    prev: usize,
    key: K,
    value: V,
    next: usize,
}

pub struct LruCache<K, V> {
    // entries: HashMap<K, V>,
    // keys_to_vec: HashMap,
    capacity: usize,
    locations: HashMap<K, usize>,
    head: usize,
    tail: usize,
    values: Vec<Entry<K, V>>,
}

impl <K, V > LruCache<K, V> 
where K: Hash + Eq + Clone{
    pub fn new(capacity: usize) -> Self{
        Self {
            capacity,
            locations: HashMap::new(),
            values: Vec::new(),
            head: NULL,
            tail: NULL,
        }
    }

    pub fn put(&mut self, key: K, value: V){
        let k2 = key.clone();
        let val = self.locations.remove(&key);

        if let Some(removed_index) = val {
            
            if self.len() == 1{
                self.values[0].value = value;
            } else {
                let removed_entry = &mut self.values[removed_index];
            
                removed_entry.value = value;

                self.move_to_head(removed_index);
            }

            
        } else {
            if self.values.is_empty(){
                self.head = 0;
                self.tail = 0;
                self.add_first(key, value)
            } else {
                if self.values.len() == self.capacity {
                    let old_index = self.remove_tail();
                    self.add_to_head_indexed(old_index, key, value)
                } else {
                    self.add_to_head(key, value);
                }
            }
        }

        self.locations.insert(k2, self.head);

        // println!("{:?}", self.values);
    }

    pub fn get(&self, key: &K) -> Option<&V>{
        if let Some(index) = self.locations.get(key){
            Some(&self.values[*index].value)
        } else{
            None
        }
    }

    pub fn pop(&mut self, key: &K) -> Option<V>{
        todo!();
    }
    
    pub fn capacity(&self) -> usize{
        self.capacity
    }

    fn move_to_head(&mut self, index: usize){

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

        let entry = &mut self.values[index];

        let head_index = self.head;
        entry.prev = NULL;
        entry.next = head_index;

        self.head = index;
        self.values[head_index].prev = self.head;

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

    fn len(&self) -> usize{
        self.values.len()
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
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_opt_eq(cache.get(&1), 10);
        assert_opt_eq(cache.get(&2), 20);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_put_update() {
        let mut cache: LruCache<String, Vec<u8>> = LruCache::new(1);
        cache.put("1".to_string(), vec![10, 10]);
        cache.put("1".to_string(), vec![10, 19]);
        assert_opt_eq(cache.get(&"1".to_string()), vec![10, 19]);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_expire_lru() {
        let mut cache: LruCache<String, String> = LruCache::new(2);
        cache.put("foo1".to_string(), "bar1".to_string());
        cache.put("foo2".to_string(), "bar2".to_string());
        cache.put("foo3".to_string(), "bar3".to_string());
        assert!(cache.get(&"foo1".to_string()).is_none());
        cache.put("foo2".to_string(), "bar2update".to_string());
        cache.put("foo4".to_string(), "bar4".to_string());
        assert!(cache.get(&"foo3".to_string()).is_none());
    }

    // #[test]
    // fn test_pop() {
    //     let mut cache= LruCache::new(2);
    //     cache.put(1, 10);
    //     cache.put(2, 20);
    //     assert_eq!(cache.len(), 2);
    //     let opt1 = cache.pop(&1);
    //     assert!(opt1.is_some());
    //     assert_eq!(opt1.unwrap(), 10);
    //     assert!(cache.get(&1).is_none());
    //     assert_eq!(cache.len(), 1);
    // }

    // #[test]
    // fn test_change_capacity() {
    //     let mut cache = LruCache::new(2);
    //     assert_eq!(cache.capacity(), 2);
    //     cache.put(1, 10);
    //     cache.put(2, 20);
    //     cache.change_capacity(1);
    //     assert!(cache.get(&1).is_none());
    //     assert_eq!(cache.capacity(), 1);
    // }

    // #[test]
    // fn test_to_string() {
    //     let mut cache = LruCache::new(3);
    //     cache.put(1, 10);
    //     cache.put(2, 20);
    //     cache.put(3, 30);
    //     assert_eq!(cache.to_string(), "{3: 30, 2: 20, 1: 10}".to_string());
    //     cache.put(2, 22);
    //     assert_eq!(cache.to_string(), "{2: 22, 3: 30, 1: 10}".to_string());
    //     cache.put(6, 60);
    //     assert_eq!(cache.to_string(), "{6: 60, 2: 22, 3: 30}".to_string());
    //     cache.get(&3);
    //     assert_eq!(cache.to_string(), "{3: 30, 6: 60, 2: 22}".to_string());
    //     cache.change_capacity(2);
    //     assert_eq!(cache.to_string(), "{3: 30, 6: 60}".to_string());
    // }

    // #[test]
    // fn test_clear() {
    //     let mut cache = LruCache::new(2);
    //     cache.put(1, 10);
    //     cache.put(2, 20);
    //     cache.clear();
    //     assert!(cache.get(&1).is_none());
    //     assert!(cache.get(&2).is_none());
    //     assert_eq!(cache.to_string(), "{}".to_string());
    // }
}