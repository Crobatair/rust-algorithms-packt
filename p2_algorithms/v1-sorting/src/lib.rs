// declare a bubble sort function
// of type T with PartialOrd
// param must be mut and & *new memory location of T

use std::fmt::Debug;

// O(n^2)
pub fn bubble_sort<T:PartialOrd + Debug> (v:&mut [T]){
    for p in 0 ..v.len(){
        println!("{:?}", v);

        // is array sorted
        let mut sorted = true;

        for i in 0..(v.len() -1) -p {
            if v[i] > v[i+1] {
                v.swap(i, i+1);
                // array is not sorted
                sorted = false;
            }
        }
        println!("{:?}", v);

        if sorted {
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4,6,1,2,5,3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }
}
