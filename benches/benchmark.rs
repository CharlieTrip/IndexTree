use criterion::{criterion_group, criterion_main, Criterion};

use index_tree::IndexTree;
use std::hint::black_box;

const OBJ: [u8; 11] = [10, 15, 80, 72, 1, 0, 81, 72, 30, 8, 100];
const DIM: [usize; 6] = [10, 10, 10, 10, 10, 10];
const _DIM: [usize; 6] = [1, 1, 1, 1, 1, 1_000_000];

pub fn get(v: Vec<usize>) -> Vec<u8> {
  v.iter().map(|&i| OBJ[i % 10]).collect()
}

pub fn cycle_for() {
  for i1 in 0..DIM[0] {
    for i2 in 0..DIM[1] {
      for i3 in 0..DIM[2] {
        for i4 in 0..DIM[3] {
          for i5 in 0..DIM[4] {
            for i6 in 0..DIM[5] {
              let _ = get(vec![i1, i2, i3, i4, i5, i6]);
            }
          }
        }
      }
    }
  }
}

pub fn cycle_gen_tree() {
  let tree = IndexTree::new(&DIM.to_vec(), &vec![]);
  let _ = tree.check();
}

pub fn cycle_index_tree() {
  let mut tree = IndexTree::new(&DIM.to_vec(), &vec![]);
  while tree.check() {
    let _ = tree.get();
    let _ = tree.inc();
  }
}

fn bench_cycle_for(c: &mut Criterion) {
  c.bench_function("bench_cycle_for", |b| {
    b.iter(|| {
      black_box(cycle_for());
    });
  });
}

fn bench_gen_tree(c: &mut Criterion) {
  c.bench_function("bench_gen_tree", |b| {
    b.iter(|| {
      black_box(cycle_gen_tree());
    });
  });
}

fn bench_index_tree(c: &mut Criterion) {
  c.bench_function("bench_index_tree", |b| {
    b.iter(|| {
      black_box(cycle_index_tree());
    });
  });
}

criterion_group!(benches, bench_cycle_for, bench_gen_tree, bench_index_tree,);
criterion_main!(benches);
