use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Input test.");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret number is: {}", secret_number);

    println!("Input a string.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("If input is not valid this shows for the crash.");

    println!("Input was: {}", guess);

	match guess.cmp(&secret_number){
	    Ordering::Less => println!("Too low."),
		Ordering::Greater => println!("Too high."),
		Ordering::Equal => println!("Correct"),
	}
}
