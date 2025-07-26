//! Example
//!

#![deny(missing_docs)]
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

/// Example func for benching
fn fib(n: u64) -> u64 {
	match n {
		0 => 1,
		1 => 1,
		n => fib(n - 1) + fib(n - 2),
	}
}

/// A benchmark
pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("fib 20", |b| b.iter(|| fib(black_box(20))));
}

/// Groups
mod groups {
	#![allow(missing_docs)]
	use super::*;
	criterion_group!(benches, criterion_benchmark);
}
criterion_main!(groups::benches);
