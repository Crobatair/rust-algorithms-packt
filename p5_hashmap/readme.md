# Information related to Hashmaps and BTreeMaps.


This module contains information related to how to build a HashMap with custom traits, hash and get & get_mut
Also to add trait hash to a custom element.


---

On [v1_hmap](../../main/p5_hash_map/v1_hmap/src/hasher.rs) we used to create a struct hasher.
Because we want to use our hasher with trait Hash, we must implement the methods:
**write** and **finish**.


With **write**, we specify an algoritm or something to ensure that a bit array will be encoded.

With **finish**, we retrieve the hashed value of an registry, we can specify convert **write** to a u64 as on example..

---

On [v2_buckets](../../main/p5_hash_map/v2_buckets/src/main.rs) we create a struct called __BucketList__ to hold a seed, len and buckets.

The __seed __ is used to create a unique hash for each bucket. The buckets are pushed to vector with a unique hash, and if we use the same function to generate a hash from a reference, we get the hashed key to find a object on a Vector of objects.


----

On [v3_finish_hashmap](../../main/p4_connected_nodes/v3_finish_hashmap/src/main.rs) we start to crete our HashMap.

Is created a new struct __HMap__ that will hold all values of BucketList<K,V>. But why? Why we should create a wrapper Struct that holds out HashMap of data. If we see out BucketList definition, it will hold all information of the hashmap, so why we used to create an new Struct?

The answer is that we must ensure that our HashMap __(BucketList)__ *its a Vector of Vector of <V> that have a capacity... and if we can add any number of data to out hashmap we must create a new BucketList that holds for a some time the current data.

The solution is have on our struct, other BucketList that will holds the information of any added __<K, V>__, and we must ensure that the process of grow the original Vec *copy all data of Vec to a new Vec with more capacity, will no opaque performance.

- When we insert, we test if <K> is on our __main__ HashMap, and update the value. We update the pointer, because we have the same type and lenght of memory, and after adding to HashMap __insert__, we dont need to use more, we just use the pointer to memory.

```rust
  pub fn insert(&mut self, k: K, v: V){
    // more code

    // get_mut from BucketList to get a mutable of Value
    if let Some(iv) = self.main.get_mut(&k){
        *iv = v;
        return;
    }

    // more code
  }

```
- but we muts consider that we are using a second HashMap that will holds the data that need to be added *Because out vec can reach his own capcity, we need to:

```rust
  pub fn insert(&mut self, k: K, v: V){
    // more code

    // get_mut from BucketList to get a mutable of Value
    if let Some(iv) = self.grow.get_mut(&k){
        *iv = v;
        return;
    }

   // more code
  }

```
- if we have reach a number of inserts on our main BucketList, we must start adding to __grow__ and, move all Grow buckets to our main Bucket.


```rust
  pub fn insert(&mut self, k: K, v: V){
    // more code

    if self.n_moved > 0 {
      self.grow.push(k, v);
      self.move_bucket();
      return;
    }
   // more code
  }

```

- But, if we dont moved anything. we just add to Main. We test if len of our main HashMap is not too highier that a BSIZE/2 and we move our buckets,  to increase capacity.


```rust
  pub fn insert(&mut self, k: K, v: V){
    // more code

    if self.main.push(k, v) > ( BSIZE/2 ){
        self.move_bucket();
    }
   // more code
  }

```


move_buckets just handle the grow of main bucket, or pass grow buckets to main buckets.

we used swap mem to just pass the mem ref to grow, that will became our main BucketList.
