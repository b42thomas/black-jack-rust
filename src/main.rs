// //! A CLI blackjack game.

mod card;
// mod display;
// mod math;
// mod player;
// mod round;
// mod utils;
// use crate::*;
// use display::*;
// use math::*;
// use round::*;
// use utils::*;
mod ui;


fn main() {
	ui::clear(); // The terminal should clear
	let mut card_deck: Vec<card::card::Card> = card::card::Card::card_pack();
	for card in card_deck {
		println!("{}", card);
	}
		
	println!("Thanks for Playing! :)")
}
