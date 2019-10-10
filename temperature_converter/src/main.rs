use std::io;

fn main() {
    println!("Welcome to the temperature converter");
    loop {
        println!("If you want to convert from Fahrenheit type: F");
        println!("If you want to convert from Celsius type: C");

        let mut input_type = String::new();
        io::stdin().read_line(&mut input_type)
            .expect("Failed to read the line");
    
        let input_type: char = match input_type.trim().parse() {
            Ok(res) => res,
            Err(_) => continue,
        };

        println!("Write the temperature value");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("Failed to read the line");
            
        let temp: f32 = match temp.trim().parse() {
            Ok(res) => res,
            Err(_) => {
                println!("Not valid number - number is not float!");
                continue
                },
        };

        let result: f32 = if input_type == 'F' {
            (temp - 32.) * 5./9. 
        } else if input_type == 'C' {
            (temp *9./5.) + 32.
        } else {
            println!("You must type 'F' or 'C'!");
            continue
        };
        println!("Result is {}", result);
    }
}
