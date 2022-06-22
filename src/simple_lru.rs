use std::collections::HashMap;

pub struct Cache<K, V> {
    capacity: usize,
    entries: HashMap<K, V>,
    keys: Vec<K>,
}

impl <K, V> Cache<K, V> 
where K: std::hash::Hash + std::cmp::Eq + std::clone::Clone + std::marker::Copy{
    pub fn new(capacity: usize) -> Self{
        Self {
            capacity,
            entries: HashMap::new(),
            keys: Vec::new(),
        }
    }


    pub fn set(&mut self, key: K, value: V){
        if self.keys.contains(&key){
            self.keys.retain(|x| x != &key)
        } else {
            if self.keys.len() >= self.capacity{
                let key = self.keys.remove(0);
                self.entries.remove(&key);
            }
        }
        self.keys.push(key);
        self.entries.insert(key, value);
    }

    pub fn get(&mut self, key: &K) -> Option<&V>{
        self.entries.get(key)
    }
}