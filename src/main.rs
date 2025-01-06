use rand::Rng; // enable random generator
fn main() {
    println!("\n== Number Guess Game ==\n");

    // create random num generator
    let mut rng = rand::thread_rng(); 

    // generate a number between 1 to 101
    let random_number: i32 = rng.gen_range(1..101); 
    println!("Random number: {}", random_number);
    println!("\n=======================\n");

}
