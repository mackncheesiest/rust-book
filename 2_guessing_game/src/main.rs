use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You didn't input a number, but whatever I'm not going to crash or anything, baka"); 
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small!"),
            Ordering::Greater => println!("Your guess was too big!"),
            Ordering::Equal => {
                println!("Your guess was correct!");
                break;
            }
        }
    }
}
