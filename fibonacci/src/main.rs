use std::io;

fn main() {
    println!("Welcome to Fibonacci number generator");
    loop {
        println!("Which number from the series you want - type the integer");
        let mut nth = String::new();
        io::stdin().read_line(&mut nth)
            .expect("Failed to read line");

        let nth: u64 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let mut count = nth;
        let mut previous : u64 = 0;
        let mut temp : u64;
        let mut result : u64 = if nth == 0 {
            0
        } else {
            1
        };
        while count > 1 {
            temp = result;
            result = result + previous;
            previous = temp;
            count = count - 1;
        }
        println!("{}st/nd/th Fibonacci number is {}", nth, result);
    }    
}

//fn fib(x: u64) -> u64 {
  //  if x == 0 {
    //    0
    //} else if x == 1{
    //    1
    //} else {
    //    fib(x-1) + fib(x-2)
    //}
//}
