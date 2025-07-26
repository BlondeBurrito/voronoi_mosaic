//! Helper functions
//!

use rand::prelude::*;

pub mod dice;

/// Generate a random value in the range [low..high], i.e. inclusive of low and exclusive of high
pub fn random_integer(low: u32, high: u32) -> u32 {
	let mut rng = rand::rng();
	rng.random_range(low..high)
}
