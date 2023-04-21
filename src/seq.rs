use std::collections::{BinaryHeap, HashMap};
use crate::util::{Grid, Node, expand, Log};

pub fn astar<'a>(
    init_state: &Grid,
    end_state: &Grid,
    h_func: fn(&Grid, &Grid) -> i32,
) -> Option<Node<'a>> {
    let mut log = Log::new();
    let mut start = Node::new(init_state.clone());
    start.calc_cost(end_state, h_func);
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed: HashMap<Grid, Node> = HashMap::new();
    open.push(start);

    while !open.is_empty() {
        let n = open.pop().unwrap();
        // println!("pop n, f: {}, h: {}", n.f, n.h);
        if n.state == *end_state {
            return None;
        }

        let nodes = expand(&n, end_state, h_func);
        closed.insert(n.state.clone(), n);

        for c in nodes {
            if closed.contains_key(&c.state) {
                let node = closed.get(&c.state).unwrap();
                if c.g < node.g {
                    closed.remove(&c.state);
                } else {
                    continue;
                }
            }
            open.push(c);
        }
    }
    None
}
