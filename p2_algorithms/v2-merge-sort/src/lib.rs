use std::fmt::Debug;

pub fn merge_sort<T:PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half
    // sort the right half O( n * ln(n) )
    // bring the sorted halfs together   O(n)

    // if vector len is less or eq than 1, vector is sorted
    if v.len() <= 1{
        return v;
    }
    
    // create return var with capacity of current vector    
    let mut res: Vec<T> = Vec::with_capacity(v.len());    


    // split vector and store result        ( Left side of split remains on original vector capacity does not change )
    let last_split = v.split_off(v.len()/2);
    
    let first_half = merge_sort(v);            // sort_merge each side of original vector
    let last_half = merge_sort(last_split);    // Right hand of split remains on new variable

    let mut first_it = first_half.into_iter(); // vector to iterable
    let mut last_it = last_half.into_iter();   // vector to iterable
    let mut first_peek = first_it.next();      // first element of iterable
    let mut last_peek = last_it.next();        // first element of last hand of v

    loop {
        match first_peek {  // perform a match for element on first_peek


            //  match Some -> will perform a match on last_peek   ( access to first side, and access to second hand )
            Some(ref a_val) => match last_peek {
                // first -> Some of T ie. i32   
                Some(ref b_val) => {
                    // last -> Some of T ie. i32   
                    // so, if last < first ( a > b )
                    // the result must be, a before b
                    // because a is the highier on first hand,  if a is lower than b, B must be pushed to lower position on result
                    // so, a needs to test if on other hand exist other lower value.
                    if b_val < a_val {
                        // b is lower, so is pushed to result
                        // take from array from top, and use unwrap()
                        // take returns a Some / None, unwrap, return the value of Some<T>
                        // now, peek a next value on last hand vector
                        res.push(last_peek.take().unwrap());
                        last_peek = last_it.next();
                    } else {
                        // if a is lower, push to res, take from array *to not iterate over it 
                        // get Some of val
                        // peek next val of pirst hand
                        res.push(first_peek.take().unwrap());
                        first_peek = first_it.next();
                    }
                }
                // Second next does not have value, so push first next to result, and return result
                None => {
                    res.push(first_peek.take().unwrap());
                    // is extended all first iterator, because is not needed to compare to nothing
                    res.extend(first_it);
                    return res;
                }
            }
            // None means, first iterator does not contains any value
            None => {
                // but, if last_peek contains a value, add to result, extend resutl with last iterable, and return res
                // v -> [1,2,3,4,5]  first = None, last = Some( v-> [5] )
                // v -> [1,2,3,4,5, 5] and return
                if let Some(b_val) = last_peek {
                    res.push(b_val);
                }
                res.extend(last_it);
                return res;
            }
        }
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let v = vec![8,11,5,2,6,1];
	let sorted = merge_sort(v);
        assert_eq!(sorted, vec![1, 2, 5, 6, 8, 11]);
    }
}
