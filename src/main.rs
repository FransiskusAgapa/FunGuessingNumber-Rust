use rand::Rng; // enable random generator
use std::io;

fn main() {
    println!("\n== Number Guess Game ==\n");
    /* 
    TODO
       -> add loop to keep asking if input is not valid 
       -> let user keep guessing until the guess is correct and give hint to go lower/higher
       -> finalize just by using 1 function to call inside main 'start_game()'
    
    */

    // get raw/input as string
    let raw_guess_limit = prompt_guess_limit();

    // trim and validate whether input is a number 
    let (is_guess_limit_number, guess_limit) = trim_validate_input(raw_guess_limit);

    if is_guess_limit_number {
        // specify number to guess 
        let num_to_guess: i32 = generate_random_number_guess(guess_limit);

        // prompt user to input a number guess 
        let raw_user_guess = prompt_user_guess();
        
        // trim and validate user guess
        let(is_user_guess_number, user_guess) = trim_validate_input(raw_user_guess);

        if is_user_guess_number{
            if num_to_guess == user_guess {
                println!("You got it!");
            } else {
                println!("Not quite, try again later. ");
            }
        } else {
            println!("Your guess has to be a number between 1 to {}", guess_limit);
        }
    } else {
        println!("C'mon, your guess limit has to be a number");
    }
    println!("\n=======================\n");
} 

// prompt user to input 'guess_limit' then  return it
fn prompt_guess_limit() -> String {
    // prompt input for number guess limit
    println!("Input your number guess limit:");

    // user input raw / read as string
    let mut raw_guess_limit  = String::new();

    // read user input and put it into 'raw_guess_limit'
    io::stdin()
    .read_line(&mut raw_guess_limit)
    .expect("Failed to read");

    // return value
    raw_guess_limit
}

// trim and validate if whether 'string_input' is a number
fn trim_validate_input(string_input: String) -> (bool, i32) {
    match string_input.trim().parse::<i32>(){
        Ok(number) => (true,number),
        Err(_) => (false, 0),
    }
}

// prompt user to input a guess number that's read as string
fn prompt_user_guess() -> String {
    // prompt input for user guess
    println!("Input your guess: ");

    // read user guess as string 
    let mut raw_user_guess = String::new();

    // read user guess and put it into 'raw_user_guess'
    io::stdin()
    .read_line(&mut raw_user_guess)
    .expect("Failed to read");

    // return 
    raw_user_guess
}

// generate random number 
fn generate_random_number_guess(guess_limit: i32) -> i32 {
    // random number generator
    let mut rng = rand::thread_rng(); 

    //  generate random num between 1 to 'guess_limit'
    let number_to_guess = rng.gen_range(1..guess_limit);

    // return 
    number_to_guess
}