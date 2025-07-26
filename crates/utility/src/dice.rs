//! Rolling dice
//!

use rand::random_range;

/// Access to a series of dice rolls
pub struct Dice;

impl Dice {
	/// Roll between 1 and 2, inclusive
	pub fn roll_d2() -> u8 {
		random_range(1..=2)
	}
	/// Roll between 1 and 4, inclusive
	pub fn roll_d4() -> u8 {
		random_range(1..=4)
	}
	/// Roll between 1 and 6, inclusive
	pub fn roll_d6() -> u8 {
		random_range(1..=6)
	}
	/// Roll between 1 and 8, inclusive
	pub fn roll_d8() -> u8 {
		random_range(1..=8)
	}
	/// Roll between 1 and 12, inclusive
	pub fn roll_d12() -> u8 {
		random_range(1..=12)
	}
	/// Roll between 1 and 20, inclusive
	pub fn roll_d20() -> u8 {
		random_range(1..=20)
	}
	/// Roll between 1 and 100, inclusive
	pub fn roll_d100() -> u8 {
		random_range(1..=100)
	}
}
