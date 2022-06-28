# fast-lru

A fast, 100% safe, stack based least recently used cache. 

fast-lru uses a stack based array to store all the values, in conjunction 
with a hashmap to store the keys. It gaurentees `O(1)` time complexity 
for all operations, including `get()`, `put()`, `get_mut()`, and `pop()`.

