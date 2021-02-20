use std::fmt::Debug;
//[un_use_document]

// import b_rand
mod b_rand;

// algorithm
// Move first element to correct plave
// Everything lower should be before it
// Everything hight should be after it
// return it's location
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    // using a rand number
    let mut p = b_rand::rand(v.len());
    // place rand on position 0
    v.swap(p, 0);
    // p = 0
    p = 0;
    // ignore first, and iterate for each val
    for i in 1..v.len() {
        // if next val is lower than pivot val, pivot val must be after that value
        if v[i] < v[p] {
            v.swap(p + 1, i); // next val = i      12 {p}, 6 {i}, 10, 5  => 12 {p}, 1{i} *on first it, p+1 == i
                              // on next it,       6 {p}, 12{p+1}, 5{i} => so,  6->5->12
            v.swap(p, p + 1); // and...            5 -> 6 -> 12
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v); // find pivot
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p); // split in separated elms
    quick_sort(a); // quick sort
    quick_sort(&mut b[1..]); // middle already sorted *becasuse pivot of start, is correctly places
}

struct RawSend<T>(*mut [T]);

unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quicksort<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);

    // Use of unsafe because v can dissapear and became a dangling memory ref
    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quicksort(&mut *raw_s.0);
        });
        threaded_quicksort(&mut b[1..]);

        handle.join().ok();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut v);
        for x in 0..v.len() {
            assert!((v[p] < v[x]) == (p < x))
        }
        //assert_eq!(v, vec![1, 3, 4, 6, 11, 13]);
    }

    #[test]
    fn test_quick_sort() {
        // Unordered test
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        //ordered test
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_threaded_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        threaded_quicksort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
        panic!();
    }
}
