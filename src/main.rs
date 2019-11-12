use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess a number beteween 1 & 100.");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	loop {

    	println!("Input a number.");

    	let mut guess = String::new();

    	io::stdin().read_line(&mut guess)
        	.expect("If input is not valid this shows for the crash.");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please input a number.");
				continue;
			}
		};

    	println!("Input was: {}", guess);

		match guess.cmp(&secret_number) {
	    	Ordering::Less => println!("Too low."),
			Ordering::Greater => println!("Too high."),
			Ordering::Equal => {
				println!("You win.");
				break;
			}
		}
	}
}
