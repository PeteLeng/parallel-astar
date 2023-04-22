#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::{Rng, SeedableRng};
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

mod dpa;
mod seq;
mod util;
use util::{expand, man_dist, print_path, Grid, Node};

fn main() {
    println!("Hello, world!");
    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let end_state = Grid::rand_with_seed(5, 13, 69);
    println!("end:\n{}", end_state);

    let mut r = rand::rngs::StdRng::seed_from_u64(10);
    let actions: Vec<(i32, i32)> = (0..169).map(|_| moves[r.gen_range(0..4)]).collect();
    let start_state = end_state.do_actions(actions);
    println!(
        "dist from end state: {d}",
        d = man_dist(&start_state, &end_state)
    );
    println!("start:\n{}", start_state);

    // let mut start = Node::new(start_state);
    // start.calc_cost(&end_state, man_dist);
    // for node in expand(&start, &end_state, man_dist) {
    //     println!("dist from end state: {d}", d = node.f);
    //     println!("{}", node.state);
    // }

    // let mut h: BinaryHeap<Node> = BinaryHeap::new();
    // h.push(start);
    // for _ in 0..200 {
    //     if let Some(n) = h.pop() {
    //         // println!("pop n with dist: {d}", d = n.f);
    //         let nodes = expand(&n, &end_state, man_dist);
    //         for node in nodes {
    //             // println!("    push n with dist: {d}", d = node.f);
    //             h.push(node);
    //         }
    //     }
    // }
    // let node = h.pop().unwrap();
    // println!("backstrace:");
    // println!("{}", node.state);
    // println!("{:?}", *node.prev_actions);

    println!("seq:");
    let start = Instant::now();
    let n = seq::astar(&start_state, &end_state, man_dist).unwrap();
    println!("{}", n.state);
    println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    let dur = start.elapsed();
    println!("time: {:?}", dur);
    println!("path taken:");
    // print_path(&n);

    println!("dpa:");
    let start = Instant::now();
    let n = dpa::astar(&start_state, &end_state, man_dist).unwrap();
    println!("{}", n.state);
    println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    let dur = start.elapsed();
    println!("time: {:?}", dur);
}
