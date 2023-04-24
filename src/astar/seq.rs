use std::collections::{BinaryHeap, HashMap};
use crate::util::{Grid, Node, expand, Log};

pub fn astar(init_state: &Grid, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) -> Option<Node> {
    let mut log = Log::new();
    let mut start = Node::new(init_state.clone());
    start.calc_cost(end_state, h_func);
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed: HashMap<Grid, Node> = HashMap::new();
    open.push(start);

    while !open.is_empty() {
        log.iter_cnt += 1;
        let n = open.pop().unwrap();
        // println!("pop n, f: {}, h: {}", n.f, n.h);
        if n.state == *end_state {
            println!("#iter: {}", log.iter_cnt);
            return Some(n);
        }

        let nodes = expand(&n, end_state, h_func);
        closed.insert(n.state.clone(), n);

        for node in nodes {
            if closed.contains_key(&node.state) {
                let closed_node = closed.get(&node.state).unwrap();
                if node.g < closed_node.g {
                    closed.remove(&node.state);
                } else {
                    continue;
                }
            }
            open.push(node);
        }
    }
    None
}
