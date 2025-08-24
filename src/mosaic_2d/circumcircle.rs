//! Describes a circumcircle - a circle derived from the three vertices of a
//! triangle such that all vertices sit along the circumference of the circle
//!

use bevy::prelude::*;

/// Describes a circle which has three vertices of a triangle sat along its circumference
pub struct Circumcircle {
	/// Centre of the circle
	circumcentre: Vec2,
	/// Circle radius sqaured
	radius_sqaured: f32,
}
impl Circumcircle {
	/// From triangle vertices describe the properties of a circumcircle
	///
	/// If an edge length between vertices is zero then a circumcircle cannot be found
	pub fn new(vertex_a: Vec2, vertex_b: Vec2, vertex_c: Vec2) -> Option<Self> {
		// https://en.wikipedia.org/wiki/Circumcircle
		let denom = 2.0
			* ((vertex_a.x * (vertex_b.y - vertex_c.y))
				+ (vertex_b.x * (vertex_c.y - vertex_a.y))
				+ (vertex_c.x * (vertex_a.y - vertex_b.y)));

		if denom != 0.0 {
			let centre_x = ((vertex_a.x.powf(2.0) + vertex_a.y.powf(2.0))
				* (vertex_b.y - vertex_c.y)
				+ (vertex_b.x.powf(2.0) + vertex_b.y.powf(2.0)) * (vertex_c.y - vertex_a.y)
				+ (vertex_c.x.powf(2.0) + vertex_c.y.powf(2.0)) * (vertex_a.y - vertex_b.y))
				/ denom;

			let centre_y = ((vertex_a.x.powf(2.0) + vertex_a.y.powf(2.0))
				* (vertex_c.x - vertex_b.x)
				+ (vertex_b.x.powf(2.0) + vertex_b.y.powf(2.0)) * (vertex_a.x - vertex_c.x)
				+ (vertex_c.x.powf(2.0) + vertex_c.y.powf(2.0)) * (vertex_b.x - vertex_a.x))
				/ denom;

			let circumcentre = Vec2::new(centre_x, centre_y);
			let radius_sqaured = (circumcentre - vertex_a).length_squared();
			Some(Circumcircle {
				circumcentre,
				radius_sqaured,
			})
		} else {
			warn!("Failed to generate circumcircle");
			None
		}
	}
	/// Get the centre of the circumcircle
	pub fn get_centre(&self) -> &Vec2 {
		&self.circumcentre
	}
	/// Get the radius of the circumcircle
	pub fn get_radius_sqaured(&self) -> f32 {
		self.radius_sqaured
	}
	/// Check if a point is within the circumcircle
	pub fn is_point_within_circle(&self, point: &Vec2) -> bool {
		//TODO does this need a tolarance factor like spheres to handle values extremely close to one another???
		// (y - center_y)^2 + (x - center_x)^2 < radius^2
		let left =
			(point.y - self.circumcentre.y).powf(2.0) + (point.x - self.circumcentre.x).powf(2.0);
		let right = self.radius_sqaured;
		left < right
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_circumcircle() {
		let v_a = Vec2::new(5.0, 0.0);
		let v_b = Vec2::new(7.0, 3.0);
		let v_c = Vec2::new(2.0, 5.0);
		//
		// a_b midpoint = (6.0, 0, 1.5)
		// a_b grad = -3 / -2
		// a_b_perp_grad = -2/3
		// a_b_perp_intercept = 1.5 - (-4) = 5.5
		//
		// b-c_midpoint = (4.5, 0, 4.0)
		// b_c_grad = -2/5
		// b_c_perp_grad = 5/2
		// b_c_perp_intercept = 4 - (11.25) = -7.25
		//
		// center_x = ( -7.25 - 5.5) / (-0.66 - 2.5) = -12.75 / (-19/6) = 4.026
		// center_z = (5.5 - (-7.25 * -4/15)) / ( 1 + 4/15) = 2.815
		//
		let circumcircle = Circumcircle::new(v_a, v_b, v_c).unwrap();
		let actual_center = Vec2::new(4.0263157, 2.8157895);
		let actual_radius = 8.876732;
		assert!(!circumcircle.get_centre().is_nan());
		assert_eq!(actual_center, *circumcircle.get_centre());
		assert_eq!(actual_radius, circumcircle.get_radius_sqaured());
	}
	#[test]
	fn new_circumcircle_is_none() {
		// invalid triangle vertices
		let v_a = Vec2::new(0.0, 2.0);
		let v_b = Vec2::new(0.0, 2.0);
		let v_c = Vec2::new(2.0, 3.0);
		assert!(Circumcircle::new(v_a, v_b, v_c).is_none());
	}
	#[test]
	fn point_is_within_circumcircle() {
		let v_a = Vec2::new(5.0, 0.0);
		let v_b = Vec2::new(7.0, 3.0);
		let v_c = Vec2::new(2.0, 5.0);
		let circumcircle = Circumcircle::new(v_a, v_b, v_c).unwrap();
		let point = Vec2::new(5.0, 3.0);
		assert!(circumcircle.is_point_within_circle(&point));
	}
	#[test]
	fn point_is_not_within_circumcircle() {
		let v_a = Vec2::new(5.0, 0.0);
		let v_b = Vec2::new(7.0, 3.0);
		let v_c = Vec2::new(2.0, 5.0);
		let circumcircle = Circumcircle::new(v_a, v_b, v_c).unwrap();
		let point = Vec2::new(10.0, 3.0);
		assert!(!circumcircle.is_point_within_circle(&point));
	}
	//TODO test for None in invalid triangle setup
}
