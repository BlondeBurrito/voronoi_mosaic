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
	let delaunay = Delaunay2d::compute_triangulation_2d(&points).unwrap();
	let voronoi = Voronoi2d::from_delaunay_2d(&delaunay).unwrap();

	let expected_cell_count = 3;
	assert_eq!(expected_cell_count, voronoi.get_cells().len());
}

#[test]
fn mesh_count_unclipped() {
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
	let delaunay = Delaunay2d::compute_triangulation_2d(&points).unwrap();
	let voronoi = Voronoi2d::from_delaunay_2d(&delaunay).unwrap();

	let expected_mesh_count = 3;
	assert_eq!(expected_mesh_count, voronoi.as_bevy2d_meshes().len());
}

#[test]
fn mesh_count_clipped() {
	//NB: these points are the same in 2d_meshes_clipped and
	// 3d_voronoi examples, useful for visually checking
	let points = vec![
		Vec2::new(-380.0, -380.0),
		Vec2::new(-355.0, -375.0),
		Vec2::new(-350.0, -233.0),
		Vec2::new(-241.0, -296.0),
		Vec2::new(-169.0, -201.0),
		Vec2::new(-124.0, -86.0),
		Vec2::new(-53.0, -124.0),
		Vec2::new(-94.0, -75.0),
		Vec2::new(-22.0, -35.0),
		//
		Vec2::new(366.0, -24.0),
		Vec2::new(340.0, -284.0),
		Vec2::new(285.0, -165.0),
		Vec2::new(236.0, -94.0),
		Vec2::new(156.0, -156.0),
		Vec2::new(120.0, -85.0),
		Vec2::new(99.0, -33.0),
		Vec2::new(72.0, -199.0),
		Vec2::new(16.0, -350.0),
		//
		Vec2::new(352.0, 42.0),
		Vec2::new(326.0, 107.0),
		Vec2::new(256.0, 251.0),
		Vec2::new(175.0, 365.0),
		Vec2::new(142.0, 168.0),
		Vec2::new(102.0, 72.0),
		Vec2::new(84.0, 192.0),
		Vec2::new(58.0, 247.0),
		Vec2::new(19.0, 27.0),
		//
		Vec2::new(-385.0, 36.0),
		Vec2::new(-321.0, 354.0),
		Vec2::new(-276.0, 68.0),
		Vec2::new(-244.0, 302.0),
		Vec2::new(-153.0, 168.0),
		Vec2::new(-122.0, 272.0),
		Vec2::new(-84.0, 196.0),
		Vec2::new(-63.0, 241.0),
		Vec2::new(-24.0, 202.0),
		//
		Vec2::new(399.0, 399.0),
		Vec2::new(-399.0, 399.0),
		Vec2::new(-399.0, -399.0),
		Vec2::new(399.0, -399.0),
		//
		Vec2::new(0.0, 399.0),
		Vec2::new(-399.0, 0.0),
		Vec2::new(0.0, -399.0),
		Vec2::new(399.0, 0.0),
	];
	let delaunay = Delaunay2d::compute_triangulation_2d(&points).unwrap();
	let voronoi = Voronoi2d::from_delaunay_2d(&delaunay).unwrap();
	let boundary = vec![
		Vec2::new(200.0, 200.0),
		Vec2::new(-200.0, 200.0),
		Vec2::new(-200.0, -200.0),
		Vec2::new(200.0, -200.0),
	];

	let expected_mesh_count = 18;
	assert_eq!(
		expected_mesh_count,
		voronoi.as_clipped_bevy2d_meshes(&boundary).len()
	);
}
