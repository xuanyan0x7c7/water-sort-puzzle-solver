use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;
use water_sort_puzzle_solver::bfs_solver::BFSSolver;
use water_sort_puzzle_solver::solver::{Solver, Tube};

fn solve_random(colors: usize, height: usize, empty_tubes: usize) {
    let mut rng = thread_rng();
    let mut pattern: Vec<u8> = (0..(colors * height)).map(|x| (x / height) as u8).collect();
    pattern.shuffle(&mut rng);
    let mut tubes: Vec<Tube> = vec![];
    for color in 0..colors {
        tubes.push(pattern[(color * height)..((color + 1) * height)].to_vec());
    }
    for _ in 0..empty_tubes {
        tubes.push(vec![]);
    }
    let mut solver = BFSSolver::new(height, colors, &tubes);
    solver.search();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("BFS 4*8+2", |b| b.iter(|| solve_random(black_box(4), black_box(8), black_box(2))));
    c.bench_function("BFS 4*10+2", |b| b.iter(|| solve_random(black_box(4), black_box(10), black_box(2))));
    c.bench_function("BFS 4*12+2", |b| b.iter(|| solve_random(black_box(4), black_box(12), black_box(2))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
