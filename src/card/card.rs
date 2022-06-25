use std::fmt::Display;

use crate::card::suit::Suit;
use crate::card::rank::Rank;

/// A playing Card composed of a [Suit] and a
/// [Rank].
///
/// This struct implements Display.
/// It also includes a static method [Card::card_pack] to
/// create a new card pack of 52 [Card]s,
/// made of all combinations of [Suit]s and [Rank]s
#[derive(PartialEq)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

impl Card {
	pub fn card_pack() -> Vec<Card> {
		(0..4)
			.flat_map(|s| {
				(0..13).map(move |r| Card {
					rank: Rank::from_int(r),
					suit: Suit::from_int(s),
				})
			})
			.collect()
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}{}", self.rank, self.suit)
	}

}

/***** Tests *****/

mod tests {
	#[test]
	fn create_card_pack() {
		let mut card_pack = Card::card_pack();
		let card = card_pack.pop();
		assert!(card.is_some());
		assert!(card.unwrap()
			== Card {
				rank: Rank::Ace,
				suit: Suit::Clubs
			}
		)
	}

	#[test]
    fn card_display() {
        assert_eq!(
            "9♣",
            &format!(
                "{}",
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Clubs
                }
            )
        )
	}
}