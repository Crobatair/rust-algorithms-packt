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


impl <T, E: Weighted, ID: Clone + Hash + Eq> Graph<T, E, ID>{

    // wrapper to don call explicit and create the Route object
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Route<ID>>> {
        self.shortest_path_r(Route::start_rc(from), to)
    }

    // wrapper to send as hashset the id, and get the closest object
    pub fn shortest_path_r(&self, from: Rc<Route<ID>>, to: ID) -> Option<Rc<Route<ID>>> {
        let mut toset =HashSet::new();
        toset.insert(to);
        self.closest(from, &toset)
    }

    // modified to use hashset and direct route
    pub fn closest(&self, from: Rc<Route<ID>>, to: &HashSet<ID>) -> Option<Rc<Route<ID>>> {
        let mut visited = HashSet::new();
        let mut routes = Vec::new();
        routes.push(from);
        loop {
            let c_route = routes.pop()?;
            
            if to.contains(&c_route.pos) {
                return Some(c_route);
            }

            if visited.contains(&c_route.pos) {
                continue;
            }

            visited.insert(c_route.pos.clone());
            let exits = self.data.get(&c_route.pos)?;

            for eid in &exits.1 {
                let edge = self.edges.get(eid)?;
                let npos = if edge.1 == c_route.pos {
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                let nlen = c_route.len + edge.0.weight();
                let nroute = Rc::new(Route{
                    pos: npos,
                    len: nlen,
                    path: Some(c_route.clone()), 
                });

                if routes.len() == 0 {
                    routes.push(nroute);
                    continue;
                }

                let mut iafer = routes.len() - 1;
                loop {
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


    pub fn greedy_salesman(&self, start: ID) -> Option<Rc<Route<ID>>> {
        let mut to_visit: HashSet<ID> = self.data.keys().map(|k| k.clone()).collect();
        to_visit.remove(&start);

        let mut route = Route::start_rc(start.clone());
        while to_visit.len() > 0 {
            route = self.closest(route, &to_visit)?;
            to_visit.remove(&route.pos);
        }
        self.shortest_path_r(route, start)

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

    println!("greedy salesman A = {}", g.greedy_salesman('A').unwrap());


    Ok(())
}
