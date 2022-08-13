use std::io;
use std::cmp::Ordering;
use rand::Rng;
use rand::thread_rng;

fn main() {
    let secret_number = thread_rng().gen_range(1..2);

    loop {
        println!("Guess a number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a number");

        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, can't parse");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less number"),
            Ordering::Greater => println!("High number"),
            Ordering::Equal => {
                println!("You guessed it right");
                break;
            }
        }
    }
}
