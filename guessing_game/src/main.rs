use std::io;    //include the io library from the standard library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    //println!("Secret number is {}", secret_number);
    loop {
        println!("Please input your guess.");
        //"let" - creates new variable
        //in Rust variables are inmutable by default - use "mut" to enable mutability
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,     // "_" - is a catchall value, as we just want to catch any type of error
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
