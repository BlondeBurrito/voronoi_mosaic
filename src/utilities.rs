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
/// Tests if a vertex sits inside of a polygon
///
/// NB: the polygon edges need to have their own vertices specified in a anti-clockwise order
pub fn is_vertex_within_polygon(vertex: &Vec2, polygon_edges: &Vec<(Vec2, Vec2)>) -> bool {
	let mut winding_number: i32 = 0;
	for (edge_v1, edge_v2) in polygon_edges {
		// // find the gradient of the edge
		// let dy = edge_v2.y - edge_v1.y;
		// let dx = edge_v2.x - edge_v1.x;
		// // handle the case of gradient being infinity (ie.e dx is zero)
		// if dx  == 0.0 {
		// 	// to intercept vertex-x must be smaller than edge-x, i.e to the left of it
		// 	if vertex.x <= edge_v1.x{
		// 		// to intercept the vertex-y must be within the y bounds of the edge
		// 		// we don't know the orientation so figure out cmp based on
		// 		// which edge-y is bigger than the other
		// 		if edge_v2.y > edge_v1.y {
		// 			if vertex.y > edge_v1.y && vertex.y < edge_v2.y {
		// 				// vertex to infinity will intercept
		// 				intersection_count += 1;
		// 			}
		// 		} else {
		// 			if vertex.y > edge_v2.y && vertex.y < edge_v1.y {
		// 				intersection_count += 1;
		// 			}
		// 		}
		// 	}
		// } else if dy == 0.0 {
		// 	// special case to see if vertex is directly on and running parallel to the edge
		// 	//TODO
		// } else {
		// 	// y = mx + c
		// 	let gradient = dy / dx;
		// 	let c = edge_v1.y - (gradient * edge_v1.x);
		// 	// vertex to infinity has unchanging y so plug vertex-y into the equation
		// 	// to find the x where
		// }

		// // to intercept vertex-x must be smaller than edge-x, i.e to the left of it
		// // by using "<" we're assuming that if a vertex lies on an edge directly then it is outside of the polygon
		// if vertex.x < edge_v1.x || vertex.x < edge_v2.x {
		// 	// to intercept the vertex-y must be within the y bounds of the edge
		// 	// we don't know the orientation so figure out cmp based on
		// 	// which edge-y is bigger than the other
		// 	if edge_v2.y > edge_v1.y {
		// 		if vertex.y > edge_v1.y && vertex.y < edge_v2.y {
		// 			// vertex to infinity will intercept
		// 			intersection_count += 1;
		// 		}
		// 	} else {
		// 		if vertex.y > edge_v2.y && vertex.y < edge_v1.y {
		// 			intersection_count += 1;
		// 		}
		// 	}
		// }

		// above has issues, if vertex is very close to an edge then special cases
		// arises whereby intersections could be infinite or not at all
		// Consider Winding Method: https://en.wikipedia.org/wiki/Point_in_polygon
		//
		// Sum angles between vertex and the edge for a winding number (wn)
		// wn = sum( arccos (edge1 - v)dot(edge2 - v) / abs(edge1 - v)abs(edge2 - v) )
		// if wn is non-zero then vertex is in polygon
		//
		// on the surface looks slower than intersection test as it uses inverse trigononemtry but Dan Sunday proposed an alternative way to deduce the Winding Number without the need for trig:
		// https://web.archive.org/web/20130126163405/http://geomalgorithms.com/a03-_inclusion.html

		if edge_v1.y <= vertex.y {
			// if edge crosses upwards
			if edge_v2.y > vertex.y {
				// if vertex if left of edge then increment winding number
				if is_vertex_left_of_edge(vertex, (*edge_v1, *edge_v2)) > 0.0 {
					winding_number += 1;
				}
			}
		} else {
			// if edge crosses down
			if edge_v2.y <= vertex.y {
				// if vertex is right of edge then decrement winding number
				if is_vertex_left_of_edge(vertex, (*edge_v1, *edge_v2)) < 0.0 {
					winding_number -= 1;
				}
			}
		}
	}
	winding_number != 0
}

/// Test if `vertex` is located on the left side of `edge`
///
/// NB: this is considered from the perspective of the direction of the edge
///
/// NB: to work properly the veritces that make up an edge need to be specified in an anti-clockwise order
///
/// * Value > 0 means it is on the left
/// * Value == 0 means it is on the edge
/// * Value < 0 means it is on the right
pub fn is_vertex_left_of_edge(vertex: &Vec2, edge: (Vec2, Vec2)) -> f32 {
	// // check orientation
	// if Vec2::Y.angle_to(edge.1 - edge.0).is_sign_negative() {
	// 	(edge.1.x - edge.0.x) * (vertex.y - edge.0.y) - (vertex.x - edge.0.x) * (edge.1.y - edge.0.y)
	// } else {
	// 	(edge.0.x - edge.1.x) * (vertex.y - edge.1.y) - (vertex.x - edge.1.x) * (edge.0.y - edge.1.y)
	// }

	// let dy = edge.1.y - edge.0.y;
	// let dx = edge.1.x - edge.0.x;
	// if dx == 0.0 {
	// 	// vertical line
	// 	if vertex.x < edge.0.x {
	// 		1.0
	// 	} else if vertex.x > edge.0.x {
	// 		-1.0
	// 	} else {
	// 		0.0
	// 	}
	// } else {
	// 	let m = dy / dx;
	// 	let c = edge.0.y - (m * edge.0.x);
	// 	// y - mx - c = 0
	// 	// plug vert x-y in, if non-zero then sign indicates left or right

	// 	// assumes for horizontal line, point above is positive (left), point below negative (right)
	// 	let result = vertex.y - (m * vertex.x) - c;
	// 	if result.powf(2.0) < EPSILON {
	// 		0.0
	// 	} else {
	// 		result
	// 	}
	// }
	(edge.1.x - edge.0.x) * (vertex.y - edge.0.y) - (vertex.x - edge.0.x) * (edge.1.y - edge.0.y)
}

/// Checks if a point is within the x-y limts of an edge
pub fn is_point_within_edge_range_limt(point: &Vec2, edge_start: &Vec2, edge_end: &Vec2) -> bool {
	((point.x >= edge_start.x && point.x <= edge_end.x)
		|| (point.x >= edge_end.x && point.x <= edge_start.x))
		&& ((point.y >= edge_start.y && point.y <= edge_end.y)
			|| (point.y >= edge_end.y && point.y <= edge_start.y))
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

	#[test]
	fn is_vertex_left_of_edge1() {
		let v = Vec2::new(-5.0, 3.0);
		let edge = (Vec2::new(2.0, -4.0), Vec2::new(5.0, 9.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n > 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge2() {
		let v = Vec2::new(15.0, 3.0);
		let edge = (Vec2::new(2.0, -4.0), Vec2::new(5.0, 9.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n < 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge3() {
		let v = Vec2::new(2.0, -4.0);
		let edge = (Vec2::new(2.0, -4.0), Vec2::new(5.0, 9.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n == 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge4() {
		let v = Vec2::new(-5.0, 3.0);
		let edge = (Vec2::new(5.0, 9.0), Vec2::new(2.0, -4.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n < 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge5() {
		let v = Vec2::new(15.0, 3.0);
		let edge = (Vec2::new(5.0, 9.0), Vec2::new(2.0, -4.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n > 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge6() {
		let v = Vec2::new(2.0, -4.0);
		let edge = (Vec2::new(5.0, 9.0), Vec2::new(2.0, -4.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n == 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge7() {
		let v = Vec2::new(1.0, -4.0);
		let edge = (Vec2::new(2.0, 2.0), Vec2::new(2.0, 9.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n > 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge8() {
		let v = Vec2::new(5.0, -4.0);
		let edge = (Vec2::new(2.0, 2.0), Vec2::new(2.0, 9.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n < 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge9() {
		let v = Vec2::new(2.0, 5.0);
		let edge = (Vec2::new(2.0, 2.0), Vec2::new(2.0, 9.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n == 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge10() {
		let v = Vec2::new(3.0, 6.0);
		let edge = (Vec2::new(2.0, 5.0), Vec2::new(8.0, 5.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n > 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge11() {
		let v = Vec2::new(3.0, -6.0);
		let edge = (Vec2::new(2.0, 5.0), Vec2::new(8.0, 5.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n < 0.0);
	}
	#[test]
	fn is_vertex_left_of_edge12() {
		let v = Vec2::new(3.0, 5.0);
		let edge = (Vec2::new(2.0, 5.0), Vec2::new(8.0, 5.0));
		let n = is_vertex_left_of_edge(&v, edge);
		assert!(n == 0.0);
	}

	#[test]
	fn vertex_within_polygon1() {
		// polygon
		//
		// (0, 10)         (10, 10)
		//
		//           v(5,5)
		//
		// (0,  0)         (10,  0)
		//
		let polygon_edges = vec![
			(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)),
			(Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0)),
			(Vec2::new(10.0, 10.0), Vec2::new(0.0, 10.0)),
			(Vec2::new(0.0, 10.0), Vec2::new(0.0, 0.0)),
		];
		let vertex = Vec2::new(5.0, 5.0);
		assert!(is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon2() {
		// polygon
		//
		//          v(5,11)
		// (0, 10)         (10, 10)
		//
		//
		//
		// (0,  0)         (10,  0)
		//
		let polygon_edges = vec![
			(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)),
			(Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0)),
			(Vec2::new(10.0, 10.0), Vec2::new(0.0, 10.0)),
			(Vec2::new(0.0, 10.0), Vec2::new(0.0, 0.0)),
		];
		let vertex = Vec2::new(5.0, 11.0);
		assert!(!is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon3() {
		// polygon
		//
		// (0, 10)         (10, 10)
		//
		//
		//
		// (0,  0)         (10,  0)
		//           v(5,-1)
		//
		let polygon_edges = vec![
			(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)),
			(Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0)),
			(Vec2::new(10.0, 10.0), Vec2::new(0.0, 10.0)),
			(Vec2::new(0.0, 10.0), Vec2::new(0.0, 0.0)),
		];
		let vertex = Vec2::new(5.0, -1.0);
		assert!(!is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon4() {
		// polygon
		//
		// (0, 10)         (10, 10)
		//
		//                           v(12,5)
		//
		// (0,  0)         (10,  0)
		//
		let polygon_edges = vec![
			(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)),
			(Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0)),
			(Vec2::new(10.0, 10.0), Vec2::new(0.0, 10.0)),
			(Vec2::new(0.0, 10.0), Vec2::new(0.0, 0.0)),
		];
		let vertex = Vec2::new(12.0, 5.0);
		assert!(!is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon5() {
		// polygon
		//
		//    (0, 10)         (10, 10)
		//
		// v(-1,5)
		//
		//    (0,  0)         (10,  0)
		//
		let polygon_edges = vec![
			(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)),
			(Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0)),
			(Vec2::new(10.0, 10.0), Vec2::new(0.0, 10.0)),
			(Vec2::new(0.0, 10.0), Vec2::new(0.0, 0.0)),
		];
		let vertex = Vec2::new(-1.0, 5.0);
		assert!(!is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon6() {
		// polygon
		//                 (2, 10)
		//
		//
		//      (-5, 1)              (5, 3)
		//
		//
		//                 (2, -10)
		let polygon_edges = vec![
			(Vec2::new(2.0, -10.0), Vec2::new(5.0, 3.0)),
			(Vec2::new(5.0, 3.0), Vec2::new(2.0, 10.0)),
			(Vec2::new(2.0, 10.0), Vec2::new(-5.0, 1.0)),
			(Vec2::new(-5.0, 1.0), Vec2::new(2.0, -10.0)),
		];
		let vertex = Vec2::new(-1.0, 1.0);
		assert!(is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon7() {
		// polygon
		//                 (2, 10)
		//
		//
		//      (-5, 1)              (5, 3)
		//
		//
		//                 (2, -10)
		let polygon_edges = vec![
			(Vec2::new(2.0, -10.0), Vec2::new(5.0, 3.0)),
			(Vec2::new(5.0, 3.0), Vec2::new(2.0, 10.0)),
			(Vec2::new(2.0, 10.0), Vec2::new(-5.0, 1.0)),
			(Vec2::new(-5.0, 1.0), Vec2::new(2.0, -10.0)),
		];
		let vertex = Vec2::new(10.0, 1.0);
		assert!(!is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon8() {
		// polygon
		//                 (2, 10)
		//
		//
		//      (-5, 1)              (5, 3)
		//
		//
		//                 (2, -10)
		let polygon_edges = vec![
			(Vec2::new(2.0, -10.0), Vec2::new(5.0, 3.0)),
			(Vec2::new(5.0, 3.0), Vec2::new(2.0, 10.0)),
			(Vec2::new(2.0, 10.0), Vec2::new(-5.0, 1.0)),
			(Vec2::new(-5.0, 1.0), Vec2::new(2.0, -10.0)),
		];
		let vertex = Vec2::new(-10.0, 1.0);
		assert!(!is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon9() {
		// polygon
		//                 (2, 10)
		//
		//
		//      (-5, 1)              (5, 3)
		//
		//
		//                 (2, -10)
		let polygon_edges = vec![
			(Vec2::new(2.0, -10.0), Vec2::new(5.0, 3.0)),
			(Vec2::new(5.0, 3.0), Vec2::new(2.0, 10.0)),
			(Vec2::new(2.0, 10.0), Vec2::new(-5.0, 1.0)),
			(Vec2::new(-5.0, 1.0), Vec2::new(2.0, -10.0)),
		];
		let vertex = Vec2::new(-6.0, 1.0);
		assert!(!is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn vertex_within_polygon10() {
		// polygon
		//                 (2, 10)
		//
		//
		//      (-5, 1)              (5, 3)
		//
		//
		//                 (2, -10)
		let polygon_edges = vec![
			(Vec2::new(2.0, -10.0), Vec2::new(5.0, 3.0)),
			(Vec2::new(5.0, 3.0), Vec2::new(2.0, 10.0)),
			(Vec2::new(2.0, 10.0), Vec2::new(-5.0, 1.0)),
			(Vec2::new(-5.0, 1.0), Vec2::new(2.0, -10.0)),
		];
		let vertex = Vec2::new(2.0, 10.0);
		assert!(!is_vertex_within_polygon(&vertex, &polygon_edges));
	}
	#[test]
	fn point_in_range() {
		let point = Vec2::new(3.0, 5.0);
		let edge_start = Vec2::new(1.0, 3.0);
		let edge_end = Vec2::new(7.0, 9.0);
		assert!(is_point_within_edge_range_limt(&point, &edge_start, &edge_end));
	}
	#[test]
	fn point_out_range() {
		let point = Vec2::new(-4.0, 12.0);
		let edge_start = Vec2::new(1.0, 3.0);
		let edge_end = Vec2::new(7.0, 9.0);
		assert!(!is_point_within_edge_range_limt(&point, &edge_start, &edge_end));
	}
}
