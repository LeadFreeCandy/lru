use std::{collections::HashMap, hash::Hash};


enum Entry<V> {
    Head(usize),
    Tail(usize),
    Value(usize, V, usize)
}
use Entry::*;

pub struct Cache<K, V> {
    // entries: HashMap<K, V>,
    // keys_to_vec: HashMap,
    capacity: usize,
    locations: HashMap<K, usize>,
    values: Vec<Entry<V>>,
}

impl <K, V > Cache<K, V> 
where K: std::hash::Hash + std::cmp::Eq + std::clone::Clone + std::marker::Copy{
    pub fn new(capacity: usize) -> Self{
        Self {
            capacity,
            locations: HashMap::new(),
            values: vec![Head(1), Tail(0)],
        }
    }

    pub fn set(&mut self, key: K, value: V){
        if let Some(index) = self.locations.get(&key){
            todo!()
        } else {


            if let Tail(preceding_index) = self.values[1]{

                self.values.push(Value(preceding_index, value, 1));
                let new_index = self.values.len() - 1;
                self.locations.insert(key, new_index);


                match &mut self.values[preceding_index] {
                    Head(next) => {
                        *next = new_index;
                    },
                    Value(_prev, _val, next) => {
                        *next = new_index;
                    },
                    _ => panic!(),
                }
            }

            
        }

        
    }

    pub fn get(&self, key: &K) -> Option<&V>{
        if let Some(index) = self.locations.get(key){
            if let Value(_, val, _) = &self.values[*index]{
                Some(val)
            } else {
                panic!()
            }
        } else{
            None
        }
    }
}