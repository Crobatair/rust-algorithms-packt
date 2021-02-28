mod errors;
mod traits;
mod route;

use errors::GraphErr;
use traits::Weighted;
use route::*;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub struct Graph<T, E, ID: Hash + Eq>{
    data: HashMap<ID,(T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

impl <T, E, ID: Clone+Hash + Eq> Graph<T, E, ID>{

    pub fn new()-> Self {
        Graph{
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, data: T) {
        self.data.insert(id, (data, Vec::new()));
    }

    pub fn add_edge(&mut self, ed_id:ID, from: ID, to:ID, edat: E)-> Result<(), GraphErr>{
        if ! self.data.contains_key(&from){
            return Err(GraphErr::new("'from' not in does"));
        }

        if let Some(ref mut dt) = self.data.get_mut(&to){
            self.edges.insert(ed_id.clone(),(edat, from.clone(), to.clone()));
            dt.1.push(ed_id.clone());
        } else {
            return Err(GraphErr::new("'to' not in nodes"));
        }

        self.data.get_mut(&from).unwrap().1.push(ed_id.clone());
        Ok(())
    }

}


// implement trait for E: Weighted
impl <T, E: Weighted, ID: Clone + Hash + Eq> Graph<T, E, ID>{

    // impl of shortest_path *if E with trait of Weighted
    // *for this reason, has been implemented trait Weighted on i32 on traits.rs
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Route<ID>>> {
        
        let mut visited = HashSet::new();
        let mut routes = Vec::new();

        // we need to retrieve a array of nodes on short, so we create a new array
        routes.push(Route::start_rc(from));
        
        loop {
            // pop from routes
            let c_route = routes.pop()?;
            
            // if to == current position, just return node ( direct node )
            if to == c_route.pos {
                return Some(c_route);
            }

            // if i have already visited this node, just skip ( dont iterate over visited nodes to recursive error )
            if visited.contains(&c_route.pos) {
                continue;
            }
            // insert cloned of c_route because it will consume
            visited.insert(c_route.pos.clone());

            // get from the hashmap the current element ?> because it can be not there
            let exits = self.data.get(&c_route.pos)?;

            // .1 -> from
            // .2 -> to
            for eid in &exits.1 {
                // try to get if edge if exist on edges of self 
                let edge = self.edges.get(eid)?;
                // test if from == current_position
                let npos = if edge.1 == c_route.pos {
                    // oposite side of edge to current pos ( to )
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                // sum the len of passed route with edge weight
                let nlen = c_route.len + edge.0.weight();

                // create the route object
                let nroute = Rc::new(Route{
                    pos: npos,
                    len: nlen,
                    path: Some(c_route.clone()), 
                });

                // if the route is len -> 0
                if routes.len() == 0 {
                    routes.push(nroute);
                    continue;
                }

                // insert into the list sorted
                let mut iafer = routes.len() - 1;
                loop {

                    // if routes
                    // compare lens 
                    if routes[iafer].len > nlen {
                        routes.insert(iafer+1, nroute);
                        break;
                    }
                    if iafer == 0 {
                        routes.insert(0, nroute);
                        break;
                    }
                    iafer -= 1;
                }
            }
        }
    }
}


fn main() -> Result<(), GraphErr> {

    let mut g = Graph::new();
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']{
        g.add_node(x, ());
    }
    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    g.add_edge('h', 'A', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;
    println!("{:?}",g);

    println!("shortest path A to D= {}", g.shortest_path('A', 'D').unwrap());

    Ok(())
}
