use std::time::Duration;

const ONE_SECOND:Duration = std::time::Duration::from_millis(1000);

pub fn clear_terminal() {
	// use std::thread::sleep;
	// use std::time::Instant;
	// use std::time::Duration;
	println!("Starting Blackjack in 5 seconds...");
	println!("!!Terminal will clear!!");
	for i in 0..5 {
		std::thread::sleep(ONE_SECOND);
		println!("{} seconds", 5 - i);
	}
	print!("\x1B[2J\x1B[1;1H");
}