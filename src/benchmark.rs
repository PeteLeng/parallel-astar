#![allow(unused_imports)]
use criterion::{criterion_group, criterion_main, Criterion};

mod astar;
mod utils;

use astar::{dpa, hda, seq};
use utils::{
    helpers::{man_dist, read_tests},
    structs::{AZHasher, Grid, StateHash, ZHasher},
};

fn bench_dpa(c: &mut Criterion) {
    let thread_cnt = vec![2, 4, 8, 16];
    let temps: Vec<f32> = vec![0.2, 0.4, 0.6, 0.8];

    let mut group = c.benchmark_group("dpa temp");
    group.sample_size(10);

    let mut tests = read_tests("oneplus");
    // let end = Grid::rand_with_seed(5, 69);
    // let start = end.rand_actions(150);
    let (start, end) = tests.remove(0);

    for &num_threads in thread_cnt.iter() {
        for temp in temps.clone() {
            let id = format!("dpa_t{}_p{}", num_threads, unsafe {
                (temp * 10.0).to_int_unchecked::<u32>()
            });
            group.bench_function(id, |b| {
                b.iter(|| {
                    dpa::astar(&start, &end, man_dist, num_threads, 0.4);
                })
            });
        }
    }

    group.finish();
}

fn bench_hda(c: &mut Criterion) {
    let thread_cnt = vec![1, 2, 4, 8, 16];

    let mut group = c.benchmark_group("hda");
    group.sample_size(10);

    let end_state = Grid::rand_with_seed(5, 69);
    let start_state = end_state.rand_actions(169);

    for &num_threads in thread_cnt.iter() {
        let hasher = ZHasher::new(start_state.size);
        let id = format!("hda_t{}_zorb", num_threads);
        group.bench_function(id, |b| {
            b.iter(|| {
                hda::astar(
                    &start_state,
                    &end_state,
                    man_dist,
                    num_threads,
                    hasher.clone(),
                );
            })
        });
    }

    for &num_threads in thread_cnt.iter() {
        let hasher = AZHasher::new(start_state.size);
        let id = format!("hda_t{}_azorb", num_threads);
        group.bench_function(id, |b| {
            b.iter(|| {
                hda::astar(
                    &start_state,
                    &end_state,
                    man_dist,
                    num_threads,
                    hasher.clone(),
                );
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dpa, bench_hda);
criterion_main!(benches);
