use std::io;

fn main() {
    println!("Enter target Fibonacci number: ");
    let mut line = String::new();
    let target: u32 = match io::stdin().read_line(&mut line) {
        Ok(_) => match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You didn't input a number 😠, so you get the first Fibonacci number");
                1
            }
        },
        Err(_) => {
            println!("Wow I failed to even parse your line as a string 😠, so you get the first Fibonacci number");
            1
        }
    };
    
    let mut fib_num: u64 = 1;
    let mut previous_num: u64 = 0;

    for _ in 1..target {
        fib_num = fib_num + previous_num;
        previous_num = fib_num - previous_num;
    }

    println!("Fibonacci number {} is {}", target, fib_num);
}
