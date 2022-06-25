use std::fmt::Display;

/// The four Suits of any classical card
/// game.
///
/// This enum implements PartialEq and
/// Display.
/// It also includes a static method [Suit::from_int]
/// that maps an u32 supplied as argument to one of the suits.

#[derive(PartialEq)]
pub enum Suit {
	Spades,
	Hearts,
	Diamonds,
	Clubs,
}


impl Display for Suit {
	// fmt takes in self and a formatter and return a result using the write! macro
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f, // formatter
			"{}", // the string we want to return, we are putting the suit symbol in the {}
			match self {
				Suit::Spades => "♠",
				Suit:: Hearts => "♥",
				Suit::Diamonds => "♦",
				Suit::Clubs => "♣",
			}
		)
	}
}

impl Suit {
	pub fn from_int(a: u32) -> Suit {
		match a % 4 {
			0 => Suit::Spades,
			1 => Suit:: Hearts,
			2 => Suit:: Diamonds,
			_ => Suit::Clubs
		}
	}
}

/***** Tests *****/

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_and_display_all_suits() {
		let spade = Suit::from_int(0);
		let heart = Suit::from_int(1);
		let diamond = Suit::from_int(2);
		let club = Suit::from_int(3);

        assert_eq!("♠", &format!("{}", spade));
        assert_eq!("♥", &format!("{}", heart));
        assert_eq!("♦", &format!("{}", diamond));
        assert_eq!("♣", &format!("{}", club));
	}
}