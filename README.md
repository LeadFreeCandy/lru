# fast-lru

A fast, 100% safe, stack based least recently used cache. 

fast-lru uses a stack based array to store all the values, in conjunction 
with a hashmap to store the keys. It gaurentees `O(1)` time complexity 
for all operations, including `get()`, `put()`, `get_mut()`, and `pop()`.


## Example 

This a simple example of creating a cache, adding some values, and then reading them.

```rust,no_run
use lru::LruCache

fn main() {
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
```