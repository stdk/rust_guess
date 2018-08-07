extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_line() -> io::Result<String> {
	let mut line = String::new();

	io::stdin().read_line(&mut line)?;

	Ok(line)
}

fn get_number() -> Result<u32,String> {
	match get_line() {
		Ok(line) => {
			line.trim().parse::<u32>()
			           .map_err(|e| e.to_string())
		},
		Err(_e) => Err("Cannot read line".to_string())
	}
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
    	println!("Please input your guess.");
	    match get_number() {
	    	Ok(number) => {
	    		println!("You guessed: {}", number);
	    		match number.cmp(&secret_number) {
			    	Ordering::Less => println!("Too small!"),
			    	Ordering::Greater => println!("Too big!"),
			    	Ordering::Equal => {
			    		println!("Match!");
			    		break
			    	},
			    }
	    	},
	    	Err(_e) => println!("Reading number failed: {}", _e),
	    }
	}    
}
