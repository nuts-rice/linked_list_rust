use std::collections::HashSet;

pub struct ASimpleGraph {
    adjacency_list: Vec<Vec<Edge>>,
    nodes: Vec<KeyType>
}
//Makes sense to keep the actual values, identifiers or actual objects
//In their own list and simply work with indices of the usize type
#[derive(Clone, Debug)]
struct Edge {
    weight: u32,
    node:usize,
}

fn get_node_index(&self, node: KeyType) -> Option<usize> {
    self.nodes.iter().position(|n| n == &node)
}
//Check for valid node. Looks up the IDs provided in edges parameter to find the index
pub fn set_edges(&mut self, from: KeyType, edges: Vec<(u32, KeyType)>) {
    //Filter_map of the iterator will only include elements that evaluate to Some()
    let edges: Vec<Edge> = edges.into_iter().filter_map(|e| {
	if let Some(to) = self.get_node_index(e.1){
	    Some(Edge {weight: e.0, node: to})
	} else {
	    None
	}}).collect();
    //Find index and ID through position() function of iterator trait
    match self.nodes.iter().position(|n| n == &from) {
	Some(i) => self.adjacency_list[i] = edges,
	None => {
	    self.nodes.push(from);
	    self.adjacency_list(edges)
	}
    }
}

pub fn set_nodes(&mut self, nodes: Vec<KeyType>){
    self.nodes = nodes;
    self.adjacency_list = vec![vec![]; self.node.len()]
}
//Takes care of validating user input and sees if node actually exists
pub fn connected(&self, from: KeyType, degree: usize) -> Option<HashSet<KeyType>> {
    self.nodes.iter().position(|n| n == &from).map(|i| {
	self.connected_r(i, degree).into_iter().map(|n|
	self.nodes[n].clone()).collect()
    })
}
//Recursive call can create a list of all neighbors and run the same call
//On each. Returning a set of nodes eliminates the duplicates as well
fn connected_r(&self, from: usize, degree:usize) -> HashSet<usize> {
    if degree > 0 {
	//Recursive call returns the internal repersentation (indicies)
	self.adjacency_list[from]
	    .iter()
	    .flat_map(|e| {
		let mut set = self.connected_r(e.node, degree - 1);
		set.insert(e.node);
		set
	    }).collect()
    } else {
	HashSet::new()
    }
}
                           
pub fn shortest_path(&self, from: KeyType, to: KeyType) -> Option<(u32, Vec<KeyType>)> {
    //Boiler-plate code to ensure that both source and destination nodes are nodes in graph
    let mut src = None;
    let mut dest = None;

    for(i, n) in self.nodes.iter().enumerate() {
	if n == &from {
	    src = Some(i);
	}
	if n == &to {
	    dest = Some(i);
	}
	if src.is_some() && dest.is_some() {
	    break;
	}
    }
    if src.is_some() && dest.is_some() {
	let (src, dest) = (src.unwrap(), dest.unwrap());
	//Each node gets a tentative weight assigned, which is infinite in beginning
	//Except for orgin node, which has zero cost to reach
	let mut distance: Vec<TentativeWeight> = vec![TentativeWeight::Infinite, self.nodes.len()];
	distance[src] = TentativeWeight::Number(0);
	//"Open" list contains all the nodes yet to be processed, is created using Rust's range, as it corrosponds to the indices we are working with/
	let mut open: Vec<usize> = (0..self.nodes.len()).into_iter().collect();
	//Parent array keeps track of each node's parent once the lower cost is established allows to trace back the best possible path
	let mut parent = vec![None; self.nodes.len()];
	let mut found = false;
	while !open.is_empty() {
	    //min_index is a helper function takes the current distance and returns the index of the node that is the easiest to reach next
	    let u = min_index(&distance, &open);
	    //This node is then removed from the open list
	    let u = open.remove(u);
	    //Stop if dest is reached
	    if u == dest {
		found = true;
		break; 
	    }
	    let dist = distance[u].clone();
	    //for each edge of this node, the new distance is computed and if lower, inserted into a distance list
	    for e in &self.adjacency_list[u] {
		let new_distance = match dist {
		    TentativeWeight::Number(n) => TentativeWeight::Number(n + e.weight),
		    _ => TentativeWeight::Infinite,
		};
		let old_distance = distance[e.node].clone();

		if new_distance < old_distance {
		    distance[e.node] = new_distance;
		    parent[e.node] = Some(u);
		}
	    }
	}
	//Distance array and a parent array to be prepared for returning to the caller
	if found {
	    //Trace back the path from the destination to the orgin node in the parent array, leads to the reverse optimal path between the two nodes
	    let mut path = vec![];
	    let mut p = parent[dest].unwrap();
	    path.push(self.nodes[dest].clone());
	    while p != src {
		path.push(self.nodes[p].clone());
		p = parent[p].unwrap();
	    }
	    path.push(self.nodes[src].clone());
	    
	    path.reverse();
	    let cost = match distance[dest] {
		TentativeWeight::Number(n) => n,
		_ => 0,
	    };
	    Some((cost, path))
	} else {
	    None
	}
    } else {
	None
    }
}

mod test {
    use super::ASimpleGraph;
    #[test]
    fn basics() {
	let mut Graph = ASimpleGraph::new();
    }
}
