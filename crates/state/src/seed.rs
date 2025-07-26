//! Seeded RNG
//!

use rand::{SeedableRng, random_range};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

/// Seeded rng
///
/// Example
/// ```
/// use state::seed::Seed;
/// use rand::Rng;
/// let mut seed = Seed::new_random();
/// let i = seed.get_rng_mut().random_range(0..100);
/// ```
#[derive(Serialize, Deserialize)]
pub struct Seed(ChaCha20Rng);

impl Seed {
	/// Init seed from a `u64`
	pub fn new(value: u64) -> Self {
		Seed(ChaCha20Rng::seed_from_u64(value))
	}
	/// Init seed from a thread-local random number
	pub fn new_random() -> Self {
		let s = random_range(0..u64::MAX);
		Seed::new(s)
	}
	/// Get a mutable reference to the rng
	pub fn get_rng_mut(&mut self) -> &mut ChaCha20Rng {
		&mut self.0
	}
}
