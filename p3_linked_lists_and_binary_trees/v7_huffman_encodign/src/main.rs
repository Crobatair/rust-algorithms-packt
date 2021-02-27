use std::collections::BTreeMap;

#[derive(Debug)]
pub enum HuffNode {
    Tree(Box<HuffNode>, Box<HuffNode>),
    Leaf(char),
}

pub struct HScore {
    h: HuffNode,
    s: i32,
}

impl HuffNode {


    // Get all encoded chars on Huffnode uwu
    pub fn get_encoded_objects(&self, prevs: &Vec<char>, side: Option<bool>) -> Vec<Option<(Vec<char>, char)>> {
        let mut response: Vec<Option<(Vec<char>,char)>> = Vec::new();
        match self {
            HuffNode::Tree(l, r) => {
                let mut res :Vec<char> = Vec::new();
                let mut res_ :Vec<char> = Vec::new();
                res.extend(prevs.into_iter());
                res_.extend(prevs.into_iter());

                if let Some(s) = side {
                    match s {
                        false => {
                            res.push('0');
                            res_.push('0');
                        },
                        true => {
                            res.push('1');
                            res_.push('1');
                        }
                    }
                }
                let left_side = l.get_encoded_objects(&res, Some(false));
                let right_side = r.get_encoded_objects(&res_, Some(true));
                response.extend(left_side);
                response.extend(right_side);
            },
            HuffNode::Leaf(l) => {
                let mut res :Vec<char> = prevs.clone();
                let s = side.unwrap_or(false);
                match s {
                    false => {
                        res.push('0');
                    },
                    true => {
                        res.push('1');
                    },
                }
                response.push(Some((res, l.clone())));
            }
        }
        response
    }

    pub fn iterate_over_huff(&self, str_to_decode: Option<Vec<char>>, passed: usize) -> (char, usize) {
        if let Some(to_decode) = str_to_decode.clone() {
            let slc: Vec<char> = to_decode[passed..].to_vec();
                match self {
                    HuffNode::Tree(l,r) => {
                        if let Some(first) = slc.first() {
                            match first {
                                '0' => {
                                    let res = l.iterate_over_huff(str_to_decode.clone(), passed + 1);
                                    return res;
                                },
                                '1' => {
                                    let res = r.iterate_over_huff(str_to_decode.clone(), passed + 1);
                                    return res;
                                },
                                _ => panic!("You should never get here!!!!"),
                            }
                        } else {
                            return ('*', passed);
                        }
                    },
                    HuffNode::Leaf(leaf) => {
                        return (leaf.clone(), passed);
                    }
                }

        } else { 
            panic!("{}", "you should never get here");
        }
    }


    pub fn decode(&self, encoded_string: Option<Vec<char>>) -> String {
        let mut res: String = "".to_string();
        let mut passed :usize = 0;
        if let Some(to_decode) = encoded_string.clone() {
            while passed <= to_decode.len()-1 {
                let response = self.iterate_over_huff( encoded_string.clone(), passed);
                res.push(response.0);
                passed = response.1;
            }
        }
        return res;
    }


    pub fn print_lfirst(&self, depth: i32, dir: char) {
        match self{
            HuffNode::Tree(l,r) => {
                l.print_lfirst(depth + 1, '/');
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}*", spc, dir);
                r.print_lfirst(depth +1, '\\');
            },
            HuffNode::Leaf(c) => {
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}{}", spc, dir, c);
            },
        }
    }

    // this method is used to generate the representation of char
    // as current Huffman coder, if not find, just return None
    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {

        match self {
            HuffNode::Tree(l,r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }
                if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
    
            },
            HuffNode::Leaf(nc) => {
                if c == *nc {
                    Some(Vec::new())
    
                } else { None }
            },
        }
    }
    
    // iterate over all chars on string, and convert to Huffman coder format
    pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
        let mut res = Vec::new();
        for c in s.chars(){
            let v = self.encode_char(c)?;
            res.extend(v.into_iter());
        }
        Some(res)
    }

}



pub fn build_tree(s: &str) -> HuffNode {
    let mut map = BTreeMap::new();
    
    // for each char on str
    for c in s.chars() {
        // get map value of key, from key &c and unwrap or 0 *where 0 means, no value
        let n = *map.get(&c).unwrap_or(&0);
        // inser on map, where key == c and, append 1 level *n+1
        // if char already exists, more levels will have
        map.insert(c, n+1);
    }


    // previous is converted as a BTreeMap, with letters and number of times
    // conver the map as iterator, and for each individual map, will create a HScore object

    // each key is a new Leaf *individual char, with s as s, and collect to convert to matched type
    let mut tlist :Vec<HScore> = map
        .into_iter()
        .map(|(k,s)| HScore{
            h: HuffNode::Leaf(k),
            s
        }).collect();

    // HScore{ h-> c, s-> number of times that appears }
    
    // if the number of c in tlist is >> that 1 *at least 1 chat on str
    while tlist.len() > 1 {
        // get last element of vector
        let last = tlist.len() - 1;

        // for 0 to last element - 1
        // 0.. tlist.len() - 1 - 1
        for i in 0..last - 1 {
            // if score of i is lower that previous than i
            if tlist[i].s<tlist[last-1].s {
                // most used char, will swap position with previos on Vec
                tlist.swap(i, last-1);
            }

            // if previous than current iterable, is lower than current iterable, then
            if tlist[last-1].s < tlist[last].s {
                // swap last -1 with last
                tlist.swap(last-1, last);
            }
        }

        let a_node = tlist.pop().unwrap(); // len > = 2
        let b_node = tlist.pop().unwrap();
        let nnode = HuffNode::Tree(Box::new(a_node.h), Box::new(b_node.h));
        tlist.push(
            HScore{
                h:nnode,
                s: a_node.s + b_node.s,
            }
        );
    }
    tlist.pop().unwrap().h
}



fn main() {
    println!("All HuffMan encoder is implemented with chars, to understand more easier the algorithm");
    let text = "at an apple app";
    println!("Original String -> {}", text);
    println!("Encoded HuffMan encoder representation");
    let t = build_tree(text);
    t.print_lfirst(0, '<');

    let encoded_string =  t.encode_str(text);
    println!("All encoded caracters... with their respective code representation");
    for i in t.get_encoded_objects(&Vec::new(), None){
        println!("{:?}", i);
    }

    let decoded = t.decode(encoded_string);
    println!("Decoded Original String: {:?}", decoded);

}
