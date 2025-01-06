use rand::Rng; // enable random generator
use std::io;

fn main() {
    println!("\n== Number Guess Game ==\n");

    // create random num generator
    let mut rng = rand::thread_rng(); 
    
    // prompt input for number guess limit
    println!("Input your number guess limit:");

    // raw user input
    let mut input = String::new();

    // read user input and put it into 'input'
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read");

    // trim 'input' to remove newline
    let input = input.trim();

    // print the 'input'
    println!("You entered: {}",input);

    // check if 'input' is number
    match input.parse::<f64>(){
        Ok(number) => {
            println!("Input is a number: {}",number)
        }
        Err(_) => {
            println!("Input is not a number: {}",input)
        }
    }
    
    // generate a number between 1 to 'userInput'
    let random_number: i32 = rng.gen_range(1..101); 
    println!("\nRandom number: {}", random_number);
    println!("\n=======================\n");

}
