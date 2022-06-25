use std::fmt::Display;


/// The thirteen Ranks of any classical card
/// game.
///
/// This enum implements PartialEq and
/// Display.
/// It also includes a static method [Rank::from_int]
/// that maps an u32 supplied as argument to one of the ranks.
#[derive(PartialEq)]
pub enum Rank {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace
}

impl Display for Rank {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Rank::Two => "2",
				Rank::Three => "3",
				Rank::Four => "4",
				Rank::Five => "5",
				Rank::Six => "6",
				Rank::Seven => "7",
				Rank::Eight => "8",
				Rank::Nine => "9",
				Rank::Ten => "10",
				Rank::Jack => "J",
				Rank::Queen => "Q",
				Rank::King => "K",
				Rank::Ace => "A",
			}
		)
	}
}

impl Rank {
	pub fn from_int(a: u32) -> Rank {
		match a % 13 {
			0 => Rank::Two,
			1 => Rank::Three,
			2 => Rank::Four,
			3 => Rank::Five,
			4 => Rank::Six,
			5 => Rank:: Seven,
			6 => Rank::Eight,
			7 => Rank::Nine,
			8 => Rank::Ten,
			9 => Rank::Jack,
			10 => Rank::Queen,
			11 => Rank::King,
			_ => Rank::Ace,
		}
	}
}

/***** Tests *****/

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_and_display_all_ranks() {
		let two = Rank::from_int(0);
		let three = Rank::from_int(1);
		let four = Rank::from_int(2);
		let five = Rank::from_int(3);
		let six = Rank::from_int(4);
		let seven = Rank::from_int(5);
		let eight = Rank::from_int(6);
		let nine = Rank::from_int(7);
		let ten = Rank::from_int(8);
		let jack = Rank::from_int(9);
		let queen = Rank::from_int(10);
		let king = Rank::from_int(11);
		let ace = Rank::from_int(12);

        assert_eq!("2", &format!("{}", two));
        assert_eq!("3", &format!("{}", three));
        assert_eq!("4", &format!("{}", four));
        assert_eq!("5", &format!("{}", five));
		assert_eq!("6", &format!("{}", six));
        assert_eq!("7", &format!("{}", seven));
        assert_eq!("8", &format!("{}", eight));
        assert_eq!("9", &format!("{}", nine));
		assert_eq!("10", &format!("{}", ten));
        assert_eq!("J", &format!("{}", jack));
        assert_eq!("Q", &format!("{}", queen));
        assert_eq!("K", &format!("{}", king));
		assert_eq!("A", &format!("{}", ace));
	}
}