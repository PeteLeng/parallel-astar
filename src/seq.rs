use std::collections::{BinaryHeap, HashMap};
use crate::util::{Grid, Node, expand};

pub fn astar(init_state: &Grid, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) -> Option<Node> {
    let mut start = Node::new(init_state.clone());
    start.evaluate(end_state, h_func);
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed: HashMap<Grid, Node> = HashMap::new();
    open.push(start);
    let mut stat_cnt = 0;

    while !open.is_empty() {
        let n = open.pop().unwrap();
        // println!("pop n, f: {}, h: {}", n.f, n.h);
        let nodes = expand(&n, end_state, h_func);

        if n.grid == *end_state {
            println!("states expanded: {}", stat_cnt);
            return Some(n);
        }

        closed.insert(n.grid.clone(), n);

        for c in nodes {
            stat_cnt += 1;
            if closed.contains_key(&c.grid) {
                let node = closed.get(&c.grid).unwrap();
                if c.f < node.f {
                    closed.remove(&c.grid);
                } else {
                    continue;
                }
            }
            open.push(c);
        }
    }
    None
}
