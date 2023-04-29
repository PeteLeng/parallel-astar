#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::{Rng, SeedableRng};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

mod astar;
mod utils;

use astar::{dpa, hda, seq};
use utils::helpers::{gen_tests, man_dist, read_tests};
use utils::structs::{AZHasher, Grid, Node, StateHash, ZHasher};

pub fn setup() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_files = vec!["easy", "medium", "hard"];
    let params = [(4, 30, 25, 30), (4, 30, 30, 35), (5, 30, 30, 35)];

    for i in 0..3 {
        let mut path = root.clone();
        path.push(format!("src/tests/{}.txt", test_files[i]));
        let mut f = File::create(path).expect("failed to create file.");
        let tests = gen_tests(params[i]);
        f.write_all(tests.as_bytes()).unwrap();
    }
}

pub fn test_dpa(tier: &str, num_threads: usize, temp: f32) {
    println!("dpa {} t{} {}", tier, num_threads, temp);
    let test_cases = read_tests(tier);
    let start = Instant::now();
    let mut tot_dur = Duration::new(0, 0);
    for (i, (s, e)) in test_cases.iter().enumerate() {
        let loop_start = start.elapsed();
        let end = dpa::astar(s, e, man_dist, num_threads, temp);
        let loop_end = start.elapsed();
        let dur = loop_end - loop_start;
        println!("- test {} {}: {:?}", i, end.unwrap().f, dur);
        tot_dur += dur;
    }
    println!("Total time: {:?}", tot_dur);
    println!("Average time: {:?}", tot_dur / test_cases.len() as u32);
}

pub fn test_hda(tier: &str, num_threads: usize) {
    println!("hda {} t{}", tier, num_threads);
    let test_cases = read_tests(tier);
    let start = Instant::now();
    let mut tot_dur = Duration::new(0, 0);
    for (i, (s, e)) in test_cases.iter().enumerate() {
        let hasher = ZHasher::new(s.size);
        let loop_start = start.elapsed();
        let end = hda::astar(s, e, man_dist, num_threads, hasher);
        let loop_end = start.elapsed();
        let dur = loop_end - loop_start;
        println!("- test {} {}: {:?}", i, end.unwrap().f, dur);
        tot_dur += dur;
    }
    println!("Total time: {:?}", tot_dur);
    println!("Average time: {:?}", tot_dur / test_cases.len() as u32);
}

pub fn test_zhda(tier: &str, num_threads: usize) {
    println!("abs hda {} t{}", tier, num_threads);
    let test_cases = read_tests(tier);
    let start = Instant::now();
    let mut tot_dur = Duration::new(0, 0);
    for (i, (s, e)) in test_cases.iter().enumerate() {
        let hasher = AZHasher::new(s.size);
        let loop_start = start.elapsed();
        let end = hda::astar(s, e, man_dist, num_threads, hasher);
        let loop_end = start.elapsed();
        let dur = loop_end - loop_start;
        println!("- test {} {}: {:?}", i, end.unwrap().f, dur);
        tot_dur += dur;
    }
    println!("Total time: {:?}", tot_dur);
    println!("Average time: {:?}", tot_dur / test_cases.len() as u32);
}

pub fn test_seq(tier: &str) {
    println!("seq {}", tier);
    let test_cases = read_tests(tier);
    let start = Instant::now();
    let mut tot_dur = Duration::new(0, 0);
    for (i, (s, e)) in test_cases.iter().enumerate() {
        let loop_start = start.elapsed();
        seq::astar(s, e, man_dist);
        let loop_end = start.elapsed();
        let dur = loop_end - loop_start;
        println!("- test {}: {:?}", i, dur);
        tot_dur += dur;
    }
    println!("Total time: {:?}", tot_dur);
    println!("Average time: {:?}", tot_dur / test_cases.len() as u32);
}

pub fn filter_tests() -> () {
    let tiers = ["easy", "medium", "hard"];
    let mut res = vec![];
    for tier in tiers {
        let test_cases = read_tests(tier);
        let start = Instant::now();
        for (i, (s, e)) in test_cases.iter().enumerate() {
            let loop_start = start.elapsed();
            seq::astar(s, e, man_dist);
            let loop_end = start.elapsed();
            let dur = loop_end - loop_start;
            // println!("test {}: {:?}", i, dur);
            if dur > Duration::new(1, 0) {
                println!("add {}.{}", tier, i);
                res.push((s.clone(), e.clone()));
            }
        }
    }
    let tests = serde_json::to_string(&res).unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/tests/oneplus.txt");
    let mut f = File::create(path).expect("failed to create file.");
    f.write_all(tests.as_bytes()).unwrap();
}

fn main() {
    // println!("Hello, world!");
    // let end_state = Grid::rand_with_seed(6, 13, 69);
    // println!("end:\n{}", end_state);

    // let start_state = end_state.rand_actions(200);
    // println!(
    //     "dist from end state: {d}",
    //     d = man_dist(&start_state, &end_state)
    // );
    // println!("start:\n{}", start_state);

    // println!("seq:");
    // let start = Instant::now();
    // let n = seq::astar(&start_state, &end_state, man_dist).unwrap();
    // println!("{}", n.state);
    // println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);

    // println!("dpa with temp 0.8:");
    // let start = Instant::now();
    // let n = dpa::astar(&start_state, &end_state, man_dist, 8, 0.8).unwrap();
    // println!("{}", n.state);
    // println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);

    // println!("dpa with temp 0.4:");
    // let start = Instant::now();
    // let n = dpa::astar(&start_state, &end_state, man_dist, 8, 0.4).unwrap();
    // println!("{}", n.state);
    // println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);

    // println!("hda with zobrist:");
    // let start = Instant::now();
    // let hasher = ZHasher::new(start_state.size);
    // let n = hda::astar(&start_state, &end_state, man_dist, 8, hasher).unwrap();
    // println!("{}", n.state);
    // println!("f: {}, g: {}, h: {}", n.f, n.g, n.h);
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);

    // let hasher = ZHasher::new(5);
    // let states: Vec<Grid> = (0..100).map(|i| end_state.rand_actions(i)).collect();
    // let start = Instant::now();
    // states.iter().map(|s| s.hash_with(&hasher)).for_each(drop);
    // // .for_each(|v| println!("hash value: {:0>32b}", v));
    // let dur = start.elapsed();
    // println!("time: {:?}", dur);

    // setup();
    // filter_tests();

    // let tests = read_tests("hard");
    // for (i, test) in tests.iter().enumerate() {
    //     let (start, end) = test;
    //     // println!("start:\n{}", start);
    //     // println!("end:\n{}", end);
    //     println!("test case {}", i);
    //     println!("man dist: {}\n", man_dist(start, end));

    //     // if i > 10 {
    //     //     break;
    //     // }
    // }

    // test_seq("oneplus");

    // test_dpa("oneplus", 2, 0.6);
    // test_dpa("oneplus", 4, 0.6);
    test_dpa("oneplus", 8, 0.6);
    // test_dpa("oneplus", 16, 0.6);

    // test_hda("oneplus", 2);
    // test_hda("oneplus", 4);
    test_hda("oneplus", 8);
    // test_hda("oneplus", 16);

    // test_zhda("oneplus", 2);
    // test_zhda("oneplus", 4);
    test_zhda("oneplus", 8);
    // test_zhda("oneplus", 16);
}
