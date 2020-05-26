#![feature(test)]

extern crate test;

use flatset::LevelOrder;

use test::{black_box, Bencher};

const SIZE: usize = 1_000_000;
const SAMPLES: usize = 1_000_000;
const INTERVAL: usize = SIZE / SAMPLES;

fn create_vector() -> Vec<usize> {
    (0..SIZE).collect()
}

#[bench]
fn bench_branchful_binary_search(b: &mut Bencher) {
    let vec = create_vector();
    b.iter(|| {
        for sample in 0..SAMPLES {
            let needle = sample * INTERVAL;
            black_box(vec.binary_search(black_box(&needle)).is_err());
        }
    });
}

#[bench]
fn bench_branchless_binary_search(b: &mut Bencher) {
    let vec = create_vector();
    b.iter(|| {
        for sample in 0..SAMPLES {
            let needle = sample * INTERVAL;
            black_box(vec.branchless_binary_search(black_box(&needle)).is_err());
        }
    });
}

#[bench]
fn bench_branchful_level_order_search(b: &mut Bencher) {
    let mut vec = create_vector();
    vec.level_order();
    b.iter(|| {
        for sample in 0..SAMPLES {
            let needle = sample * INTERVAL;
            black_box(vec.branchful_level_order_search(black_box(&needle)).is_ok());
        }
    });
}

#[bench]
fn bench_branchless_level_order_search(b: &mut Bencher) {
    let mut vec = create_vector();
    vec.level_order();
    b.iter(|| {
        for sample in 0..SAMPLES {
            let needle = sample * INTERVAL;
            black_box(
                vec.branchless_level_order_search(black_box(&needle))
                    .is_ok(),
            );
        }
    });
}
