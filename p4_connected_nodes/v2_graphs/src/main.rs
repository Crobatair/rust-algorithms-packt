

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct GraphErr{
    mess: String,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr {
            mess: s.to_string(),
        }
    }
}


// Mappointer based
#[derive(Debug)]
pub struct Graph<T, E, ID: Hash + Eq>{
    // data contains a hashmap, each element with an ID as Key
    // and each contains a ( data, array of ids
    data: HashMap<ID,(T, Vec<ID>)>,
    // each edge contains a ID, and a tuple of ( E, ID, ID)
    // where ID, ID is 'from' and ID is 'to'
    // {ID, (E, from, to)}
    edges: HashMap<ID, (E, ID, ID)>,
}

// any ID must implement trait eq, hash and Clone
impl <T, E, ID: Clone+Hash + Eq> Graph<T, E, ID>{

    pub fn new()-> Self {
        Graph{
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, data: T) {
        // because id is an ID, it will insert a new on hashmap, it element is duplicated, it will not added
        self.data.insert(id, (data, Vec::new()));
    }

    // will add and edge on node
    // return Ok or GraphErr type
    pub fn add_edge(&mut self, ed_id:ID, from: ID, to:ID, edat: E)-> Result<(), GraphErr>{
        // if 'from' dont exist on data, because parent node not exist
        if ! self.data.contains_key(&from){
            return Err(GraphErr::new("'from' not in does"));
        }

        // test if data contains to edge, if not return a Err() of type GraphErr
        if let Some(ref mut dt) = self.data.get_mut(&to){
            // get as mut reference because we need to increse array of  references on child graph
            // added new edge reference
            // all cloned because they be reused
            self.edges.insert(ed_id.clone(),(edat, from.clone(), to.clone()));
            // add to array of references
            dt.1.push(ed_id.clone());
        } else {
            return Err(GraphErr::new("'to' not in nodes"));
        }

        // if from and to exists, add added the edge and  reference,
        // then, get the reference of from *prev used to
        // then push to refs edge id
        self.data.get_mut(&from).unwrap().1.push(ed_id.clone());
        Ok(())
    }

}

fn main() -> Result<(), GraphErr> {

    let mut g = Graph::new();
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']{
        g.add_node(x, ());
    }
    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'A', 10)?;
    g.add_edge('d', 'H', 'B', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    g.add_edge('h', 'A', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;
    println!("{:?}",g);

    Ok(())
}
