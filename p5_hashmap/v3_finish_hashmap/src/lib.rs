mod hasher;
use std::hash::Hash;
use std::borrow::Borrow;

use hasher::hash;

const BSIZE: usize = 8;

const BGROW: usize = 8;

// a Bucketlist, contains a seed, len a buckets *seed is to use on hashs
#[derive(Debug)]
pub struct BucketList<K, V>{
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K,V)>>,
}

impl <K: Hash + Eq, V> BucketList<K, V> {

    // instance a new object of type BucketList
    fn new() -> Self {
        BucketList {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }   
    }

    // usize returned how big chosen buck
    // add to bucket
    fn push(&mut self, k:K, v:V) -> usize {

        // because we are using a hash to identify elements by key
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();

        //h is used to push to elements
        self.buckets[h].push((k, v));

        // increse bucket size
        self.len += 1;
        // return new bucket size
        self.buckets[h].len()
    }

    // KB is used to specify that KB must have trait Hash, EQ, Sized
    // its to get bucket
    // we only have the borrow ref, but not the object itself
    // so we compare the borrowed ref to K with a borrowed KB
    fn get<KB>(&self, k: &KB) -> Option<&V> where K:Borrow<KB>, KB: Hash + Eq + ?Sized{
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &self.buckets[h]{
            // if the key iquals to borrow ref to key
            if k == ik.borrow() {
                return Some(iv);
            }
        }
        None
    }

    // same as get, mut used to retrieve a mutable of V, K
    fn get_mut<KB>(&mut self, k: &KB) -> Option<&mut V> where K:Borrow<KB>, KB: Hash + Eq + ?Sized{
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &mut self.buckets[h]{
            // its wraped as &K .borrow
            // compare the borrowed of K ( because recive a &KB ) with the current on List og Buckets lists
            // and those are type ( K, V ) not borrowed as ( & KB ), so we use (ik as &K ) and borrow it to
            // compare the matching type borrows and can return the mut of v on  bucket lists
            if k == (ik as &K).borrow() {
                return Some(iv);
            }
        }
        None
    }

    fn bucket(&mut self, n:usize) -> Option<Vec<(K, V)>> {
        if n >= self.buckets.len() {
            return None;
        }
        let mut res = Vec::new();
        std::mem::swap(&mut res, &mut self.buckets[n]);
        self.len -= res.len();
        Some(res)
    }

    // set n buckets
    fn set_buckets(&mut self, n:usize){
        for _ in self.buckets.len()..n{
            self.buckets.push(Vec::new());
        }
    }
}
//////////////////////////////////////// 
// created the HMap to be used
#[derive(Debug)]
pub struct HMap<K, V>{
    n_moved: usize,
    // declared to be the current main bucketlist in use... to avoid colision of use main while adding some buckets
    main: BucketList<K, V>,
    // declared to hold temporary buckets
    grow: BucketList<K, V>,
}

// impl new to any K that traits Hash and Eq
impl <K: Hash + Eq, V> HMap<K, V> {
    pub fn new() -> Self {
        HMap {
            n_moved: 0,
            main: BucketList::new(),
            grow: BucketList::new(),
        }
    }

    // insert to main of hashmap
    pub fn insert(&mut self, k: K, v: V){
        // if k exists on main, then put v with new data
        // as standar, is changed the reference point with new, and returned
        if let Some(iv) = self.main.get_mut(&k){
            *iv = v;
            return;
        }

        // if not exists on main, it can be on grow list, because remains to move to main array
        if let Some(iv) = self.main.get_mut(&k){
            *iv = v;
            return;
        }

        // this means that we have some elemetns are on trasit?
        // and we push to grow, and call move buckets, to make grow into main
        if self.n_moved > 0 {
            // we have started move to bigger bucket list
            self.grow.push(k, v);
            self.move_bucket();

            return;
        }

        // if all other fails...
        // element must be added
        // so we call push to main, and it returbs a usize
        // if the main is higher than half of growing capacity, will move buckets 
        if self.main.push(k, v) > ( BSIZE/2 ){
            self.move_bucket();
        }


    }

    // get the borrow optional for Key
    pub fn get<KR>(&self, kr: &KR) -> Option<&V> where K:Borrow<KR>, KR: Hash + Eq + ?Sized{
        // because we only wants the not mut part of object, we can shorten with that 
        self.main.get(kr).or_else(||self.grow.get(kr))
    }

    // get the value element as a mutable reference.
    pub fn get_mut<KR>(&mut self, kr: &KR) -> Option<&mut V> where K:Borrow<KR>, KR: Hash + Eq + ?Sized{
        if let Some(b) = self.main.get_mut(kr){
            return Some(b);
        }
        self.grow.get_mut(kr)
    }

    // just in case that can be needed
    pub fn len(&self)-> usize {
        self.main.len + self.grow.len
    }

    // move bucket method
    pub fn move_bucket(&mut self){
        // if you dont have any to move, just create buckets to grow main hashmap *high cost op, when the hashmap grows, so space already reserved to grow BGROW/
        if self.n_moved == 0 {
            self.grow.set_buckets(self.main.buckets.len() + BGROW);
        }
        
        // i will be not owned by main when used
        if let Some(b) = self.main.bucket(self.n_moved){
            for (k, v) in b {
                self.grow.push(k, v);
            }
            self.n_moved += 1;
            return;
        }
        // Grow, became new main list, and main bucket is consumed and grow became an empty bucket
        std::mem::swap(&mut self.main, &mut self.grow);
        self.n_moved = 0;
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
