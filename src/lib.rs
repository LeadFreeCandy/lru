pub mod simple_lru;
// pub use simple_lru::*;

pub mod fast_lru;
// pub use fast_lru::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test(){
        let mut cache = simple_lru::Cache::new(2);

        cache.set(1, 1);
        cache.set(2, 2);
        

        assert_eq!(cache.get(&1), Some(&1));
        assert_eq!(cache.get(&2), Some(&2));
    }

    #[test]
    fn second_test(){
        let mut cache = fast_lru::Cache::new(3);

        cache.set(1, 2);
        cache.set(2, 2);
        cache.set(3, 3);
        cache.set(2, 5);
        cache.set(1, 1);
        cache.set(4, 4);


        

        assert_eq!(cache.get(&1), Some(&1));
        assert_eq!(cache.get(&2), Some(&5));
        assert_eq!(cache.get(&3), None);
    }
}

