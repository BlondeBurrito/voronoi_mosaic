//! Measure calculating the 3d Delaunay
//!

#![allow(missing_docs)]
use bevy::prelude::*;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rand::{SeedableRng, seq::IteratorRandom};
use rand_chacha::ChaCha20Rng;
use std::hint::black_box;
use voronoi_mosaic::prelude::*;

/// Create the required data before benchmarking
fn prepare_data() -> Vec<Vec3> {
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
		let z_range = std::ops::Range {
			start: 0,
			end: 10000,
		};
		let x = x_range.choose(&mut rng_seed).unwrap();
		let y = y_range.choose(&mut rng_seed).unwrap();
		let z = z_range.choose(&mut rng_seed).unwrap();
		let point = Vec3::new(x as f32, y as f32, z as f32);
		if !points.contains(&point) {
			points.push(point);
		}
	}
	points
}

/// Call the code to benchmark
fn init(points: &mut [Vec3]) {
	let _data = Delaunay3d::compute_triangulation_3d(points);
}
/// Benchmark
pub fn criterion_benchmark(c: &mut Criterion) {
	let mut data = prepare_data();
	let mut group = c.benchmark_group("3d");
	group.significance_level(0.1).sample_size(100);
	group.throughput(Throughput::Bytes(data.len() as u64));
	group.bench_function("3d_delaunay", |b| b.iter(|| init(black_box(&mut data))));
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
