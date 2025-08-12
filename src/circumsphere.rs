//! A circumsphere is a sphere which contains a polyhedron and each vertex of
//! the polyhedron touches the surface of the sphere
//!
//! In this case we restrict computations to those of tetrahedrons

use bevy::prelude::*;

/// Describes a sphere whereby the vertices of a polyhedron sit upon its
/// surface
pub struct Circumsphere {
	/// Centre of the sphere
	circumcentre: Vec3,
	/// Sphere radius squared
	radius_squared: f32,
}

impl Circumsphere {
	/// From a series of tetrahedron vertices calculate the [Circumsphere] if
	/// it exists
	pub fn new(vertex_a: Vec3, vertex_b: Vec3, vertex_c: Vec3, vertex_d: Vec3) -> Option<Self> {
		let (a_x, a_y, a_z) = (vertex_a.x, vertex_a.y, vertex_a.z);
		let (b_x, b_y, b_z) = (vertex_b.x, vertex_b.y, vertex_b.z);
		let (c_x, c_y, c_z) = (vertex_c.x, vertex_c.y, vertex_c.z);
		let (d_x, d_y, d_z) = (vertex_d.x, vertex_d.y, vertex_d.z);

		// just use matrices: https://mathworld.wolfram.com/Circumsphere.html

		let a = Mat4::from_cols(
			Vec4::new(a_x, b_x, c_x, d_x),
			Vec4::new(a_y, b_y, c_y, d_y),
			Vec4::new(a_z, b_z, c_z, d_z),
			Vec4::ONE,
		)
		.determinant();

		if a != 0.0 {
			let a_len_sq = vertex_a.length_squared();
			let b_len_sq = vertex_b.length_squared();
			let c_len_sq = vertex_c.length_squared();
			let d_len_sq = vertex_d.length_squared();

			let det_x = Mat4::from_cols(
				Vec4::new(a_len_sq, b_len_sq, c_len_sq, d_len_sq),
				Vec4::new(a_y, b_y, c_y, d_y),
				Vec4::new(a_z, b_z, c_z, d_z),
				Vec4::ONE,
			)
			.determinant();
			let det_y = -Mat4::from_cols(
				Vec4::new(a_len_sq, b_len_sq, c_len_sq, d_len_sq),
				Vec4::new(a_x, b_x, c_x, d_x),
				Vec4::new(a_z, b_z, c_z, d_z),
				Vec4::ONE,
			)
			.determinant();
			let det_z = Mat4::from_cols(
				Vec4::new(a_len_sq, b_len_sq, c_len_sq, d_len_sq),
				Vec4::new(a_x, b_x, c_x, d_x),
				Vec4::new(a_y, b_y, c_y, d_y),
				Vec4::ONE,
			)
			.determinant();

			let c = Mat4::from_cols(
				Vec4::new(a_len_sq, b_len_sq, c_len_sq, d_len_sq),
				Vec4::new(a_x, b_x, c_x, d_x),
				Vec4::new(a_y, b_y, c_y, d_y),
				Vec4::new(a_z, b_z, c_z, d_z),
			)
			.determinant();

			let x = det_x / (2.0 * a);
			let y = det_y / (2.0 * a);
			let z = det_z / (2.0 * a);

			let radius_squared = (det_x.powf(2.0) + det_y.powf(2.0) + det_z.powf(2.0)
				- (4.0 * a * c))
				/ (4.0 * a.powf(2.0));

			Some(Circumsphere {
				circumcentre: Vec3::new(x, y, z),
				radius_squared,
			})
		} else {
			None
		}
	}
	/// Get the centre of the circumsphere
	pub fn get_centre(&self) -> &Vec3 {
		&self.circumcentre
	}
	/// Get the radius sqaured of the circumsphere
	pub fn get_radius_squared(&self) -> f32 {
		self.radius_squared
	}
	/// Is the `point` position within the sphere
	pub fn is_point_within_sphere(&self, point: &Vec3) -> bool {
		//TODO is there a way of informing tolerance based on the density/closeness of the data set?
		// due to nature of floating points we introduce a
		// tolerance factor to a handle cases where
		// the point delta is extremely close to the
		// radius squared
		let tolerance = 0.001;
		//(x - x_c)^2 + (y - y_c)^2 + (z - z_c)^2 > r^2
		(point.x - self.get_centre().x).powf(2.0)
			+ (point.y - self.get_centre().y).powf(2.0)
			+ (point.z - self.get_centre().z).powf(2.0)
			< (self.get_radius_squared() * (1.0 - tolerance))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_circumsphere() {
		let vertex_a = Vec3::new(10.0, 0.0, 0.0);
		let vertex_b = Vec3::new(0.0, 0.0, 20.0);
		let vertex_c = Vec3::new(-10.0, 0.0, 0.0);
		let vertex_d = Vec3::new(0.0, 10.0, 0.0);

		let c_wrap = Circumsphere::new(vertex_a, vertex_b, vertex_c, vertex_d);
		assert!(c_wrap.is_some());
		let c = c_wrap.unwrap();
		assert_eq!(Vec3::new(0.0, 0.0, 7.5), *c.get_centre());
		assert_eq!(156.25, c.get_radius_squared());
	}
	#[test]
	fn point_is_within_circumsphere() {
		let vertex_a = Vec3::new(10.0, 0.0, 0.0);
		let vertex_b = Vec3::new(0.0, 0.0, 20.0);
		let vertex_c = Vec3::new(-10.0, 0.0, 0.0);
		let vertex_d = Vec3::new(0.0, 10.0, 0.0);

		let c = Circumsphere::new(vertex_a, vertex_b, vertex_c, vertex_d).unwrap();
		let point = Vec3::new(3.0, 5.0, 1.0);
		assert!(c.is_point_within_sphere(&point));
	}
	#[test]
	fn point_is_not_within_circumsphere() {
		let vertex_a = Vec3::new(10.0, 0.0, 0.0);
		let vertex_b = Vec3::new(0.0, 0.0, 20.0);
		let vertex_c = Vec3::new(-10.0, 0.0, 0.0);
		let vertex_d = Vec3::new(0.0, 10.0, 0.0);

		let c = Circumsphere::new(vertex_a, vertex_b, vertex_c, vertex_d).unwrap();
		let point = Vec3::new(0.0, -15.0, 0.0);
		assert!(!c.is_point_within_sphere(&point));
	}
}
