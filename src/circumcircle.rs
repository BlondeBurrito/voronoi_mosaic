//! Describes a circumcircle - a circle derived from the three vertices of a
//! triangle such that all vertices sit along the circumference of the circle
//!

use bevy::prelude::*;

/// Describes a circle which has three vertices of a triangle sat along its circumference
pub struct Circumcircle {
	/// Centre of the circle
	circumcentre: Vec2,
	/// Circle radius
	radius: f32,
}
impl Circumcircle {
	/// From triangle vertices describe the properties of a circumcircle
	///
	/// If an edge length between vertices is zero then a circumcircle cannot be found
	pub fn new(vertex_a: Vec2, vertex_b: Vec2, vertex_c: Vec2) -> Option<Self> {
		// // find the center of the circumcircle,
		// // bisecting two sides of the triangle and
		// // taking perpendicular lines their intersection is the centre
		// let ab_midpoint = (vertex_a + vertex_b) / 2.0;
		// let ab_gradient = (vertex_a.y - vertex_b.y) / (vertex_a.x - vertex_b.x);
		// let ab_perp_gradient = -1.0 / ab_gradient;
		// let ab_perp_line_intercept = ab_midpoint.y - (ab_perp_gradient * ab_midpoint.x);
		// //
		// let bc_midpoint = (vertex_b + vertex_c) / 2.0;
		// let bc_gradient = (vertex_b.y - vertex_c.y) / (vertex_b.x - vertex_c.x);
		// let bc_perp_gradient = -1.0 / bc_gradient;
		// let bc_perp_line_intercept = bc_midpoint.y - (bc_perp_gradient * bc_midpoint.x);
		// // find where the two perp lines intercept to find the centre
		// // y_a = mx_a + c_a, y_b = mx_b + c_b
		// // mx_a + c_a = mx_b + c_b
		// // to be same x_a == x_b
		// // m_ax - m_bx = c_b - c_a
		// let centre_x = (bc_perp_line_intercept - ab_perp_line_intercept)
		// 	/ (ab_perp_gradient - bc_perp_gradient);
		// // x_a = (y_a - c_a)/m_a,   x_b = (y_b - c_b)/m_b
		// // y_a - c_a = m_a * (y_b - c_b)/m_b
		// // y_a == y_b
		// // y = (y - c_b) * m_a/m_b + c_a
		// // y - y* m_a/m_b = c_a - c_b*m_a/m_b
		// // y = (c_a - c_b * (m_a/m_b)) / ( 1 - (m_a/m_b))
		// let centre_y = (ab_perp_line_intercept
		// 	- (bc_perp_line_intercept * (ab_perp_gradient / bc_perp_gradient)))
		// 	/ (1.0 - (ab_perp_gradient / bc_perp_gradient));
		// //

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
			let radius = (circumcentre - vertex_a).length();
			Some(Circumcircle {
				circumcentre,
				radius,
			})
		} else {
			None
		}
	}
	/// Get the centre of the circumcircle
	pub fn get_centre(&self) -> &Vec2 {
		&self.circumcentre
	}
	/// Get the radius of the circumcircle
	pub fn get_radius(&self) -> f32 {
		self.radius
	}
	/// Check if a point is within the circumcircle
	pub fn is_point_within_circle(&self, point: &Vec2) -> bool {
		// (y - center_y)^2 + (x - center_x)^2 < radius^2
		(point.y - self.circumcentre.y).powf(2.0) + (point.x - self.circumcentre.x).powf(2.0)
			< self.radius.powf(2.0)
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
		let actual_radius = 2.9793844;
		assert!(!circumcircle.get_centre().is_nan());
		assert_eq!(actual_center, *circumcircle.get_centre());
		assert_eq!(actual_radius, circumcircle.radius);
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
