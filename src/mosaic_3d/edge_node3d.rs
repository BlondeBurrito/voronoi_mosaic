//! Defines an ID based edge
//! 
//! 


/// Describes an edge where the vertices are represented by vertex IDs
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq)]
pub struct EdgeNode3d([usize; 2]);

impl PartialEq for EdgeNode3d {
	fn eq(&self, other: &Self) -> bool {
		(self.0[0] == other.0[0] && self.0[1] == other.0[1]) || (self.0[0] == other.0[1] && self.0[1] == other.0[0])
	}
}

impl EdgeNode3d {
	/// Create an [EdgeNode3d] from two vertex IDs
	pub fn new(a: usize, b: usize) -> Self {
		EdgeNode3d([a, b])
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