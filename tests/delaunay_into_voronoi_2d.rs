//! Cross module integration tests
//!

use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

#[test]
fn delaunay_into_voronoi_2d() {
	//NB: these points are the same in 2d_delaunay and
	// 3d_voronoi examples, useful for visually checking
	let points = vec![
		Vec2::new(-190.0, 90.0),
		Vec2::new(-145.0, 120.0),
		Vec2::new(-120.0, -45.0),
		Vec2::new(-60.0, -120.0),
		Vec2::new(-20.0, 190.0),
		Vec2::new(60.0, -10.0),
		Vec2::new(80.0, -190.0),
		Vec2::new(100.0, 140.0),
		Vec2::new(190.0, -60.0),
	];
	let delaunay = DelaunayData::compute_triangulation_2d(&points).unwrap();
	let voronoi = VoronoiData::from_delaunay_2d(&delaunay).unwrap();

	let expected_cell_count = 3;
	assert_eq!(expected_cell_count, voronoi.get_cells().len());
}
