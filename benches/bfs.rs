use std::iter;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;

use water_sort_puzzle_solver::*;

fn solve_random(colors: usize, height: usize, empty_tubes: usize) {
    let mut tubes: Vec<u8> = (0..(colors * height))
        .map(|x| (x / height + 1) as u8)
        .collect();
    tubes.shuffle(&mut thread_rng());
    tubes.extend(iter::repeat(0).take(empty_tubes * height));
    BFSSolver::new(height, tubes).search();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("BFS 4*8+2", |b| {
        b.iter(|| solve_random(black_box(4), black_box(8), black_box(2)))
    });
    c.bench_function("BFS 4*10+2", |b| {
        b.iter(|| solve_random(black_box(4), black_box(10), black_box(2)))
    });
    c.bench_function("BFS 4*12+2", |b| {
        b.iter(|| solve_random(black_box(4), black_box(12), black_box(2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
