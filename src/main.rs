#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::{Rng, SeedableRng};
use std::time::{Duration, Instant};

mod dpa;
mod seq;
mod util;
use util::{expand, man_dist, Grid, Node};

fn main() {
    println!("Hello, world!");
    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let g_e = Grid::rand_with_seed(5, (2, 2), 8);
    println!("end:\n{}", g_e);

    let mut r = rand::rngs::StdRng::seed_from_u64(10);
    let actions: Vec<(i32, i32)> = (0..150).map(|_| moves[r.gen_range(0..4)]).collect();
    let mut g_s = g_e.clone();
    for a in actions {
        if g_s.valid_action(a) {
            g_s = g_s.move_on_copy(a);
        }
    }
    println!("start:\n{}", g_s);

    // let mut n_s = Node::new(g_s);
    // n_s.evaluate(&g_e, man_dist);
    // println!("dist from end state: {d}", d = n_s.f);
    // for n in expand(&n_s, &g_e, man_dist) {
    //     println!("{}", n.grid);
    //     println!("dist from end state: {d}", d = n.f);
    // }

    // let mut h: BinaryHeap<Node> = BinaryHeap::new();
    // h.push(n_s);
    // for _ in 0..100 {
    //     if let Some(n) = h.pop() {
    //         println!("pop n with dist: {d}, action: {a:?}", d = n.f, a = n.action);
    //         let nodes = expand(&n, &g_e, man_dist);
    //         for c in nodes {
    //             println!("- push n with dist: {d}", d = c.f);
    //             h.push(c);
    //         }
    //     }
    // }

    println!("seq:");
    let start = Instant::now();
    let n = seq::astar(&g_s, &g_e, man_dist).unwrap();
    println!("{}", n.grid);
    println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    let dur = start.elapsed();
    println!("time: {:?}", dur);

    println!("dpa:");
    let start = Instant::now();
    dpa::astar(&g_s, &g_e, man_dist);
    let dur = start.elapsed();
    println!("time: {:?}", dur);
}
