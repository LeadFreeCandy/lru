use std::{collections::HashMap, hash::Hash};

const NULL: usize = std::usize::MAX;

#[derive(Debug)]
struct Entry<K, V>{
    prev: usize,
    key: K,
    value: V,
    next: usize,
}

pub struct Cache<K, V> {
    // entries: HashMap<K, V>,
    // keys_to_vec: HashMap,
    capacity: usize,
    locations: HashMap<K, usize>,
    head: usize,
    tail: usize,
    values: Vec<Entry<K, V>>,
}

impl <K, V > Cache<K, V> 
where K: std::hash::Hash + std::cmp::Eq + std::clone::Clone + std::marker::Copy + std::fmt::Debug, V: std::fmt::Debug{
    pub fn new(capacity: usize) -> Self{
        Self {
            capacity,
            locations: HashMap::new(),
            values: Vec::new(),
            head: NULL,
            tail: NULL,
        }
    }

    pub fn set(&mut self, key: K, value: V){
        let val = self.locations.remove(&key);

        if let Some(removed_index) = val {
            // self.values[val].val = value;
            let removed_entry = &mut self.values[removed_index];
            
            removed_entry.value = value;

            self.move_to_head(removed_index);

            
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

        self.locations.insert(key, self.head);

        // println!("{:?}", self.values);
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

    pub fn get(&self, key: &K) -> Option<&V>{
        if let Some(index) = self.locations.get(key){
            Some(&self.values[*index].value)
        } else{
            None
        }
    }
}