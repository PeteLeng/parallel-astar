use crate::utils::structs::{Grid, Node};
use std::collections::HashMap;

pub fn man_dist(g1: &Grid, g2: &Grid) -> i32 {
    // Assume grids with same size and same elements.
    let mut map = HashMap::new();
    for i in 0..g1.size.pow(2) {
        map.insert(g2.data[i as usize], g2.get_coord(i));
    }
    // println!("{:?}", map);

    let mut dist = 0;
    for i in 0..g1.size.pow(2) {
        let (x1, y1) = g1.get_coord(i);
        let &(x2, y2) = map.get(&g1.data[i as usize]).unwrap();
        dist += (x1 - x2).abs() + (y1 - y2).abs();
    }
    dist
}

pub fn expand(node: &Node, end_state: &Grid, h_func: fn(&Grid, &Grid) -> i32) -> Vec<Node> {
    let states = node.state.expand();
    states
        .into_iter()
        .map(|state| {
            let g = node.g + 1;
            let h = h_func(&state, end_state);
            let f = g + h;
            // let (x, y) = state.get_coord(state.empty_idx);
            // let (px, py) = node.state.get_coord(node.state.empty_idx);
            // let mut prev_actions = (*node.prev_actions).clone();
            // prev_actions.push((x - px, y - py));
            Node {
                state,
                f,
                g,
                h,
                // prev_actions: Box::new(prev_actions),
                // prev_node: Some(Box::new(node.clone())),
            }
        })
        .collect()
}

pub fn calc_receiver(node: &Node, num_threads: i32) -> i32 {
    1
}

// pub fn print_path(node: &Node) {
//     let mut state = node.state.clone();
//     let mut actions = (*node.prev_actions).clone();
//     println!("{}", state);
//     while let Some(act) = actions.pop() {
//         let act_rev = (-act.0, -act.1);
//         state = state.do_action(act_rev);
//         println!("{}", state);
//     }
// }

// pub fn print_path(node: &Node) {
//     let mut end = node.clone();
//     while let Some(b) = end.prev_node {
//         println!("{}", (*b).state);
//         end = *b;
//     }
// }
