//! A circumsphere is a sphere which contains a polyhedron and each vertex of
//! the polyhedron touches the surface of the sphere
//!
//! In this case we restrict computations to those of tetrahedrons

use bevy::{math::ops::sqrt, prelude::*};

/// Describes a sphere whereby the vertices of a polyhedron sit upon its
/// surface
pub struct Circumsphere {
	/// Centre of the sphere
	circumcentre: Vec3,
	/// Sphere radius
	radius: f32,
}

impl Circumsphere {
	/// From a series of tetrahedron vertices calculate the [Circumsphere] if
	/// it exists
	pub fn new(vertex_a: Vec3, vertex_b: Vec3, vertex_c: Vec3, vertex_d: Vec3) -> Option<Self> {
		// x, y, z is for centre of a coord
		//
		// consider equation of a sphere for each vertex:
		// sphere_a: (a_x - x)^2 + (a_y - y)^2 + (a_z - z)^2 = r^2
		// sphere_b: (b_x - x)^2 + (b_y - y)^2 + (b_z - z)^2 = r^2
		// sphere_c: (c_x - x)^2 + (c_y - y)^2 + (c_z - z)^2 = r^2
		// sphere_d: (d_x - x)^2 + (d_y - y)^2 + (d_z - z)^2 = r^2
		//
		// we set up a series of simultaneous equations that can be solved
		// to find the (x, y, z) centre
		//
		// (sphere_b - sphere_a) to find an expression for x
		// (b_x - x)^2 + (b_y - y)^2 + (b_z - z)^2 -(a_x - x)^2 - (a_y - y)^2 - (a_z - z)^2 == 0
		//
		//   (b_x - x)^2 == b_x^2 + x^2 - 2*b_x*x
		//   (b_y - y)^2 == b_y^2 + y^2 - 2*b_y*y
		//   (b_z - z)^2 == b_z^2 + z^2 - 2*b_z*z
		// - (a_x - x)^2 == - a_x^2 - x^2 + 2*a_x*x
		// - (a_y - y)^2 == - a_y^2 - y^2 + 2*a_y*y
		// - (a_z - z)^2 == - a_z^2 - z^2 + 2*a_z*z
		//
		// x(2*a_x - 2*b_x) + y(2*a_y - 2*b_y) + z(2*a_z -2*b_z) +
		// b_x^2 + b_y^2 + b_z^2 - a_x^2 - a_y^2 - a_z^2 == 0
		//
		// x(2*a_x - 2*b_x) ==
		// a_x^2 + a_y^2 + a_z^2 - b_x^2 -b_y^2 - b_z^2 - y(2*a_y - 2*b_y) - z(2*a_z -2*b_z)
		//
		// (sphere_c - sphere_a) to find an expression for y
		// (c_x - x)^2 + (c_y - y)^2 + (c_z - z)^2 -(a_x - x)^2 - (a_y - y)^2 - (a_z - z)^2 == 0
		//
		//   (c_x - x)^2 == c_x^2 + x^2 - 2*c_x*x
		//   (c_y - y)^2 == c_y^2 + y^2 - 2*c_y*y
		//   (c_z - z)^2 == c_z^2 + z^2 - 2*c_z*z
		// - (a_x - x)^2 == - a_x^2 - x^2 + 2*a_x*x
		// - (a_y - y)^2 == - a_y^2 - y^2 + 2*a_y*y
		// - (a_z - z)^2 == - a_z^2 - z^2 + 2*a_z*z
		//
		// x(2*a_x - 2*c_x) + y(2*a_y - 2*c_y) + z(2*a_z - 2*c_z) +
		// c_x^2 + c_y^2 + c_z^2 - a_x^2 - a_y^2 - a_z^2 == 0
		//
		// y(2*a_y - 2*c_y) ==
		// a_x^2 + a_y^2 + a_z^2 - c_x^2 -c_y^2 - c_z^2 - x(2*a_x - 2*c_x) - z(2*a_z - 2*c_z)
		//
		// (sphere_d - sphere_a) to find an expression for z
		// (d_x - x)^2 + (d_y - y)^2 + (d_z - z)^2 -(a_x - x)^2 - (a_y - y)^2 - (a_z - z)^2 == 0
		//
		//   (d_x - x)^2 == d_x^2 + x^2 - 2*d_x*x
		//   (d_y - y)^2 == d_y^2 + y^2 - 2*d_y*y
		//   (d_z - z)^2 == d_z^2 + z^2 - 2*d_z*z
		// - (a_x - x)^2 == - a_x^2 - x^2 + 2*a_x*x
		// - (a_y - y)^2 == - a_y^2 - y^2 + 2*a_y*y
		// - (a_z - z)^2 == - a_z^2 - z^2 + 2*a_z*z
		//
		// x(2*a_x - 2*d_x) + y(2*a_y - 2*d_y) + z(2*a_z - 2*d_z) +
		// d_x^2 + d_y^2 + d_z^2 - a_x^2 - a_y^2 - a_z^2 == 0
		//
		// z(2*a_z - 2*d_z) ==
		// a_x^2 + a_y^2 + a_z^2 - d_x^2 - d_y^2 - d_z^2 - x(2*a_x - 2*d_x) - y(2*a_y - 2*d_y)
		//
		// With the three expressions for (x, y, z) substituation can be used to solve them
		// NB: abreviate `a_x^2 + a_y^2 + a_z^2` as `L`
		//
		// 1): x(2*a_x - 2*b_x) ==
		// L - b_x^2 - b_y^2 - b_z^2 - y(2*a_y - 2*b_y) - z(2*a_z - 2*b_z)
		//
		// 2): y(2*a_y - 2*c_y) ==
		// L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x) - z(2*a_z - 2*c_z)
		//
		// 3): z(2*a_z - 2*d_z) ==
		// L - d_x^2 - d_y^2 - d_z^2 - x(2*a_x - 2*d_x) - y(2*a_y - 2*d_y)
		//
		//
		// Rewrite 3):
		// 3): z == (L - d_x^2 - d_y^2 - d_z^2 - x(2*a_x - 2*d_x) - y(2*a_y - 2*d_y)) / (2*a_z - 2*d_z)
		//
		// Substitute 3) into 2) to express y in terms of x
		//
		// 2_sub):
		// y(2*a_y - 2*c_y) ==
		// L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x) - ((L - d_x^2 - d_y^2 - d_z^2 - x(2*a_x - 2*d_x) - y(2*a_y - 2*d_y))(2*a_z - 2*c_z) / (2*a_z - 2*d_z))
		//
		// y(2*a_y - 2*c_y)(2*a_z - 2*d_z) ==
		// (L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x))(2*a_z - 2*d_z) -
		// ( 2*a_z*L -2*a_z*d_x^2 -2*a_z*d_y^2 - 2*a_z*d_z^2 -2*a_z*x(2*a_x - 2*d_x) -2*a_z*y(2*a_y - 2*d_y) - 2*c_z*L + 2*c_z*d_x^2 + 2*c_z*d_y^2 + 2*c_z*d_z^2 + 2*c_z*x(2*a_x - 2*d_x) + 2*c_z*y(2*a_y - 2*d_y))
		//
		// y(2*a_y - 2*c_y)(2*a_z - 2*d_z) ==
		// (L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x))(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2 + 2*a_z*x(2*a_x - 2*d_x) + 2*a_z*y(2*a_y - 2*d_y) + 2*c_z*L - 2*c_z*d_x^2 - 2*c_z*d_y^2 - 2*c_z*d_z^2 - 2*c_z*x(2*a_x - 2*d_x) - 2*c_z*y(2*a_y - 2*d_y)
		//
		// y(2*a_y - 2*c_y)(2*a_z - 2*d_z) + 2*c_z*y(2*a_y - 2*d_y) ==
		// (L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x))(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2 + 2*a_z*x(2*a_x - 2*d_x) + 2*a_z*y(2*a_y - 2*d_y) + 2*c_z*L - 2*c_z*d_x^2 - 2*c_z*d_y^2 - 2*c_z*d_z^2 - 2*c_z*x(2*a_x - 2*d_x)
		//
		// y((2*a_y - 2*c_y)(2*a_z - 2*d_z) + 2*c_z(2*a_y - 2*d_y)) ==
		// (L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x))(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2 + 2*a_z*x(2*a_x - 2*d_x) + 2*a_z*y(2*a_y - 2*d_y) + 2*c_z*L - 2*c_z*d_x^2 - 2*c_z*d_y^2 - 2*c_z*d_z^2 - 2*c_z*x(2*a_x - 2*d_x)
		//
		// y ==
		// ((L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x))(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2 + 2*a_z*x(2*a_x - 2*d_x) + 2*a_z*y(2*a_y - 2*d_y) + 2*c_z*L - 2*c_z*d_x^2 - 2*c_z*d_y^2 - 2*c_z*d_z^2 - 2*c_z*x(2*a_x - 2*d_x)) / ((2*a_y - 2*c_y)(2*a_z - 2*d_z) + 2*c_z(2*a_y - 2*d_y))
		//
		// We have equations to express `z` in terms of `y` and `x` (3)), and an equation expressing `y` in terms of `x` (2_sub))
		//
		// These quations can be subbed into 1) to find `x`
		//
		// 1_sub): x(2*a_x - 2*b_x) ==
		// L - b_x^2 - b_y^2 - b_z^2
		// - y(2*a_y - 2*b_y)
		// - z(2*a_z - 2*b_z)
		//
		// x(2*a_x - 2*b_x) ==
		// L - b_x^2 - b_y^2 - b_z^2
		// - y(2*a_y - 2*b_y)
		// - ((L - d_x^2 - d_y^2 - d_z^2 - x(2*a_x - 2*d_x) - y(2*a_y - 2*d_y)) / (2*a_z - 2*d_z))(2*a_z - 2*b_z)
		//
		// x(2*a_x - 2*b_x)(2*a_z - 2*d_z) ==
		// (L - b_x^2 - b_y^2 - b_z^2)(2*a_z - 2*d_z)
		// - y(2*a_y - 2*b_y)(2*a_z - 2*d_z)
		// - (L - d_x^2 - d_y^2 - d_z^2 - x(2*a_x - 2*d_x) - y(2*a_y - 2*d_y))(2*a_z - 2*b_z)
		//
		// x(2*a_x - 2*b_x)(2*a_z - 2*d_z) ==
		// (L - b_x^2 - b_y^2 - b_z^2)(2*a_z - 2*d_z)
		// - y(2*a_y - 2*b_y)(2*a_z - 2*d_z)
		// - (2*a_z*L - 2*a_z*d_x^2 - 2*a_z*d_y^2 - 2*a_z*d_z^2 - 2*a_z*x(2*a_x - 2*d_x) - 2*a_z*y(2*a_y - 2*d_y) - 2*b_z*L + 2*b_z*d_x^2 + 2*b_z*d_y^2 + 2*b_z*d_z^2 + 2*b_z*x(2*a_x - 2*d_x) + 2*b_z*y(2*a_y - 2*d_y))
		//
		// x(2*a_x - 2*b_x)(2*a_z - 2*d_z) ==
		// (L - b_x^2 - b_y^2 - b_z^2)(2*a_z - 2*d_z)
		// - y(2*a_y - 2*b_y)(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2
		// + 2*a_z*x(2*a_x - 2*d_x) + 2*a_z*y(2*a_y - 2*d_y)
		// + 2*b_z*L - 2*b_z*d_x^2 - 2*b_z*d_y^2 - 2*b_z*d_z^2
		// - 2*b_z*x(2*a_x - 2*d_x) - 2*b_z*y(2*a_y - 2*d_y)
		//
		// x(2*a_x - 2*b_x)(2*a_z - 2*d_z) ==
		// (L - b_x^2 - b_y^2 - b_z^2)(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2
		// + 2*a_z*x(2*a_x - 2*d_x)
		// + 2*b_z*L - 2*b_z*d_x^2 - 2*b_z*d_y^2 - 2*b_z*d_z^2
		// - 2*b_z*x(2*a_x - 2*d_x)
		// - y(2*a_y - 2*b_y)(2*a_z - 2*d_z) + 2*a_z*y(2*a_y - 2*d_y) - 2*b_z*y(2*a_y - 2*d_y)
		//
		// x(2*a_x - 2*b_x)(2*a_z - 2*d_z) ==
		// (L - b_x^2 - b_y^2 - b_z^2)(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2
		// + 2*a_z*x(2*a_x - 2*d_x)
		// + 2*b_z*L - 2*b_z*d_x^2 - 2*b_z*d_y^2 - 2*b_z*d_z^2
		// - 2*b_z*x(2*a_x - 2*d_x)
		// + y(2*a_z(2*a_y - 2*d_y) - (2*a_y - 2*b_y)(2*a_z - 2*d_z)- 2*b_z(2*a_y - 2*d_y))
		//
		// can now bring in y in terms of x
		//
		// x(2*a_x - 2*b_x)(2*a_z - 2*d_z) ==
		// (L - b_x^2 - b_y^2 - b_z^2)(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2
		// + 2*a_z*x(2*a_x - 2*d_x)
		// + 2*b_z*L - 2*b_z*d_x^2 - 2*b_z*d_y^2 - 2*b_z*d_z^2
		// - 2*b_z*x(2*a_x - 2*d_x)
		// + (2*a_z(2*a_y - 2*d_y) - (2*a_y - 2*b_y)(2*a_z - 2*d_z)- 2*b_z(2*a_y - 2*d_y)) * (((L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x))(2*a_z - 2*d_z) - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2 + 2*a_z*x(2*a_x - 2*d_x) + 2*a_z*y(2*a_y - 2*d_y) + 2*c_z*L - 2*c_z*d_x^2 - 2*c_z*d_y^2 - 2*c_z*d_z^2 - 2*c_z*x(2*a_x - 2*d_x)) / ((2*a_y - 2*c_y)(2*a_z - 2*d_z) + 2*c_z(2*a_y - 2*d_y)))
		//
		//
		// x(2*a_x - 2*b_x)(2*a_z - 2*d_z)((2*a_y - 2*c_y)(2*a_z - 2*d_z) + 2*c_z(2*a_y - 2*d_y)) ==
		// ((L - b_x^2 - b_y^2 - b_z^2)(2*a_z - 2*d_z)
		// - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2
		// + 2*a_z*x(2*a_x - 2*d_x)
		// + 2*b_z*L - 2*b_z*d_x^2 - 2*b_z*d_y^2 - 2*b_z*d_z^2
		// - 2*b_z*x(2*a_x - 2*d_x))((2*a_y - 2*c_y)(2*a_z - 2*d_z) + 2*c_z(2*a_y - 2*d_y))
		// + (2*a_z(2*a_y - 2*d_y) - (2*a_y - 2*b_y)(2*a_z - 2*d_z)- 2*b_z(2*a_y - 2*d_y)) * (((L - c_x^2 - c_y^2 - c_z^2 - x(2*a_x - 2*c_x))(2*a_z - 2*d_z) - 2*a_z*L + 2*a_z*d_x^2 + 2*a_z*d_y^2 + 2*a_z*d_z^2 + 2*a_z*x(2*a_x - 2*d_x) + 2*a_z*y(2*a_y - 2*d_y) + 2*c_z*L - 2*c_z*d_x^2 - 2*c_z*d_y^2 - 2*c_z*d_z^2 - 2*c_z*x(2*a_x - 2*d_x)))

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

			let radius = sqrt(det_x.powf(2.0) + det_y.powf(2.0) + det_z.powf(2.0) - (4.0 * a * c))
				/ (2.0 * a.abs());

			Some(Circumsphere {
				circumcentre: Vec3::new(x, y, z),
				radius,
			})
		} else {
			None
		}
	}
	/// Get the centre of the circumsphere
	pub fn get_centre(&self) -> &Vec3 {
		&self.circumcentre
	}
	/// Is the `point` position within the sphere
	pub fn is_point_within_sphere(&self, point: &Vec3) -> bool {
		//(x - x_c)^2 + (y - y_c)^2 + (z - z_c)^2 > r^2
		(point.x - self.get_centre().x).powf(2.0)
			+ (point.y - self.get_centre().y).powf(2.0)
			+ (point.z - self.get_centre().z).powf(2.0)
			< self.radius.powf(2.0)
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
		assert_eq!(12.5, c.radius);
	}
	#[test]
	fn point_is_within_circumcircle() {
		let vertex_a = Vec3::new(10.0, 0.0, 0.0);
		let vertex_b = Vec3::new(0.0, 0.0, 20.0);
		let vertex_c = Vec3::new(-10.0, 0.0, 0.0);
		let vertex_d = Vec3::new(0.0, 10.0, 0.0);

		let c = Circumsphere::new(vertex_a, vertex_b, vertex_c, vertex_d).unwrap();
		let point = Vec3::new(3.0, 5.0, 1.0);
		assert!(c.is_point_within_sphere(&point));
	}
	#[test]
	fn point_is_not_within_circumcircle() {
		let vertex_a = Vec3::new(10.0, 0.0, 0.0);
		let vertex_b = Vec3::new(0.0, 0.0, 20.0);
		let vertex_c = Vec3::new(-10.0, 0.0, 0.0);
		let vertex_d = Vec3::new(0.0, 10.0, 0.0);

		let c = Circumsphere::new(vertex_a, vertex_b, vertex_c, vertex_d).unwrap();
		let point = Vec3::new(0.0, -15.0, 0.0);
		assert!(!c.is_point_within_sphere(&point));
	}
}
