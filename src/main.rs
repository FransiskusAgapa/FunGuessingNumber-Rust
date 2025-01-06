use rand::Rng; // enable random generator
use std::io;

fn main() {
    println!("\n== Number Guess Game ==\n");

    // create random num generator
    let mut rng = rand::thread_rng(); 
    
    // prompt input for number guess limit
    println!("Input your number guess limit:");

    // raw user input
    let mut guess_limit  = String::new();

    // read user input and put it into 'input'
    io::stdin()
    .read_line(&mut guess_limit)
    .expect("Failed to read");

    // trim input to remove newline
    let guess_limit = guess_limit.trim();

    // validate if input is a number
    match guess_limit.parse::<i32>(){ // <expected_number_type>
        Ok(number) => { // input is a number
            let random_number_to_guess :i32 = rng.gen_range(1..number);
            println!("Random number generated!");
            println!("Input your guess between 1 to {}: ",random_number_to_guess);
            let mut raw_user_guess = String::new();

            // prompt user guess 
            io::stdin()
            .read_line(&mut raw_user_guess)
            .expect("Failed to read");

            // trim input to remove newline
            let raw_user_guess = raw_user_guess.trim(); 

            // validate if input is a number
            match raw_user_guess.parse::<i32>(){
                Ok(number) => {
                    if number == random_number_to_guess{
                        println!("You guessed it!");
                    } else {
                        println!("Not quite, try again later.");
                    }
                }

                Err(_) => {
                      println!("Guess input has to be a number");
                  }
                
            }
        }
        Err(_) => { // input is not a number
            println!("guess limit has to be a number");
        }
    }
   
    println!("\n=======================\n");
}
