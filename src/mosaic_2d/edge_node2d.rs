//! Defines an ID based edge
//!
//!

/// Describes an edge where the vertices are represented by vertex IDs
#[derive(Clone, Copy, Debug)]
pub struct EdgeNode2d([usize; 2]);

impl PartialEq for EdgeNode2d {
	fn eq(&self, other: &Self) -> bool {
		(self.0[0] == other.0[0] && self.0[1] == other.0[1])
			|| (self.0[0] == other.0[1] && self.0[1] == other.0[0])
	}
}

impl EdgeNode2d {
	/// Create an [EdgeNode2d] from two vertex IDs
	pub fn new(a: usize, b: usize) -> Self {
		EdgeNode2d([a, b])
	}
	/// Get the ID of vertex a
	pub fn get_vertex_a_id(&self) -> usize {
		self.0[0]
	}
	/// Get the ID of vertex b
	pub fn get_vertex_b_id(&self) -> usize {
		self.0[1]
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn equality() {
		let a = 1;
		let b = 2;

		let edge_i = EdgeNode2d::new(a, b);
		let edge_j = EdgeNode2d::new(b, a);

		assert!(edge_i == edge_j)
	}
}
