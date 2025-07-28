//! Measure calculating the 2d Voronoi
//!

#![allow(missing_docs)]
use bevy::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand::{SeedableRng, seq::IteratorRandom};
use rand_chacha::ChaCha20Rng;
use std::hint::black_box;
use voronoi_mosaic::{prelude::*, triangle_2d};

/// Create the required data before benchmarking
fn prepare_data() -> DelaunayData<triangle_2d::Triangle2d> {
	let mut rng_seed = ChaCha20Rng::seed_from_u64(123456789);

	let mut points = vec![];
	let point_count = 1000;
	while points.len() < point_count {
		let x_range = std::ops::Range {
			start: 0,
			end: 10000,
		};
		let y_range = std::ops::Range {
			start: 0,
			end: 10000,
		};
		let x = x_range.choose(&mut rng_seed).unwrap();
		let y = y_range.choose(&mut rng_seed).unwrap();
		let point = Vec2::new(x as f32, y as f32);
		if !points.contains(&point) {
			points.push(point);
		}
	}
	let data = DelaunayData::compute_triangulation_2d(&mut points);
	data.unwrap()
}

/// Call the code to benchmark
fn init(delaunay: &DelaunayData<triangle_2d::Triangle2d>) {
	let _v = VoronoiData::cells_from_delaunay_2d(delaunay);
}
/// Benchmark
pub fn criterion_benchmark(c: &mut Criterion) {
	let data = prepare_data();
	let mut group = c.benchmark_group("2d");
	group.significance_level(0.1).sample_size(1000);
	group.throughput(Throughput::Bytes(data.get().len() as u64));
	group.bench_function("2d_voronoi", |b| b.iter(|| init(black_box(&data))));
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
