//! Helper functions
//!

use std::cmp::Ordering;

use bevy::prelude::*;

/// Reorder a series of 2d vertices in-place based on their angular position around a point.
///
/// The ording is negative-angle to positive-angle
pub fn sort_vertices_2d(vertices: &mut Vec<Vec2>, point: &Vec2) {
	//TODO both vertices len squared cannot be zero
	vertices.sort_by(|a, b| {
		if let Some(ordering) = Vec2::Y
			.angle_to(*a - point)
			.partial_cmp(&Vec2::Y.angle_to(*b - point))
		{
			ordering
		} else {
			warn!("Unable to find Ordering between {} and {}", a, b);
			Ordering::Less
		}
	});
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn vertex_angular_order() {
		let mut vertices = vec![
			Vec2::new(10.0, 20.0),
			Vec2::new(10.0, 0.0),
			Vec2::new(0.0, 10.0),
			Vec2::new(20.0, 10.0),
		];
		let point = Vec2::new(10.0, 10.0);
		sort_vertices_2d(&mut vertices, &point);
		assert_eq!(
			vec![
				Vec2::new(10.0, 0.0),
				Vec2::new(20.0, 10.0),
				Vec2::new(10.0, 20.0),
				Vec2::new(0.0, 10.0)
			],
			vertices
		);
	}
}
