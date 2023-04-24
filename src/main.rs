#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::{Rng, SeedableRng};
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

mod astar;
mod utils;

use astar::{dpa, hda, seq};
use utils::helpers::man_dist;
use utils::structs::{Grid, Node};

fn main() {
    println!("Hello, world!");
    let end_state = Grid::rand_with_seed(5, 13, 69);
    println!("end:\n{}", end_state);

    let start_state = end_state.rand_actions(200);
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

    // println!("seq:");
    // let start = Instant::now();
    // let n = seq::astar(&start_state, &end_state, man_dist).unwrap();
    // println!("{}", n.state);
    // println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);
    // println!("path taken:");
    // print_path(&n);

    // println!("dpa:");
    // let start = Instant::now();
    // let n = dpa::astar(&start_state, &end_state, man_dist).unwrap();
    // println!("{}", n.state);
    // println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);

    // println!("dpa crossbeam temp 0.9:");
    // let start = Instant::now();
    // let n = dpa_cb::astar(&start_state, &end_state, man_dist, 0.9).unwrap();
    // println!("{}", n.state);
    // println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);

    // println!("dpa crossbeam temp 0.4:");
    // let start = Instant::now();
    // let n = dpa_cb::astar(&start_state, &end_state, man_dist, 0.4).unwrap();
    // println!("{}", n.state);
    // println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);

    println!("hda with zobrist:");
    let start = Instant::now();
    let n = hda::astar(&start_state, &end_state, man_dist).unwrap();
    println!("{}", n.state);
    println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    let dur = start.elapsed();
    println!("time: {:?}", dur);

    // let hasher = ZHasher::new(5);
    // let states: Vec<Grid> = (0..100).map(|i| end_state.rand_actions(i)).collect();
    // let start = Instant::now();
    // states.iter().map(|s| s.hash_with(&hasher)).for_each(drop);
    // // .for_each(|v| println!("hash value: {:0>32b}", v));
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);
}
