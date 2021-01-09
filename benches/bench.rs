#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]

use criterion_macro::criterion;
use criterion::{black_box, Criterion};

use iter_enhancement::IteratorExtend;

#[criterion]
fn bench_all150(c: &mut Criterion) {
    const SIZE: usize = 300;
    const THRESHOLD: usize = 150;
    let mut group = c.benchmark_group("bench_all150");
    group.bench_function("bench_at_least", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_simple", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least_simple(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_filter", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().filter(|&x| x % 2 == 0).count() >= black_box(THRESHOLD));
        });
    });
    group.finish();
}

#[criterion]
fn bench_all100(c: &mut Criterion) {
    const SIZE: usize = 300;
    const THRESHOLD: usize = 100;
    let mut group = c.benchmark_group("bench_all100");
    group.bench_function("bench_at_least", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_simple", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least_simple(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_filter", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().filter(|&x| x % 2 == 0).count() >= black_box(THRESHOLD));
        });
    });
    group.finish();
}

#[criterion]
fn bench_all50(c: &mut Criterion) {
    const SIZE: usize = 300;
    const THRESHOLD: usize = 50;
    let mut group = c.benchmark_group("bench_all50");
    group.bench_function("bench_at_least", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_simple", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least_simple(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_filter", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().filter(|&x| x % 2 == 0).count() >= black_box(THRESHOLD));
        });
    });
    group.finish();
}

#[criterion]
fn bench_all20(c: &mut Criterion) {
    const SIZE: usize = 300;
    const THRESHOLD: usize = 20;
    let mut group = c.benchmark_group("bench_all20");
    group.bench_function("bench_at_least", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_simple", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least_simple(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_filter", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().filter(|&x| x % 2 == 0).count() >= black_box(THRESHOLD));
        });
    });
    group.finish();
}

#[criterion]
fn bench_all10(c: &mut Criterion) {
    const SIZE: usize = 300;
    const THRESHOLD: usize = 10;
    let mut group = c.benchmark_group("bench_all10");
    group.bench_function("bench_at_least", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_simple", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().at_least_simple(black_box(THRESHOLD), |&x| x % 2 == 0));
        });
    });
    group.bench_function("bench_at_least_filter", |b| {
        let it: Vec<usize> = (0..SIZE).collect();
        b.iter(|| {
            assert!(it.iter().filter(|&x| x % 2 == 0).count() >= black_box(THRESHOLD));
        });
    });
    group.finish();
}