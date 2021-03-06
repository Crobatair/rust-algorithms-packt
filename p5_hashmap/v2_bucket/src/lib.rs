mod hasher;
use std::hash::Hash;
use std::borrow::Borrow;

use hasher::hash;


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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
